use lazy_static::lazy_static;
use scraper::{Html, Selector, ElementRef};
use serenity::model::prelude::{Channel};
use serenity::{
    prelude::*,
};
use serenity::model::prelude::{Message};
use regex::Regex;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, ExecuteWebhook};
use crate::webhook::get_or_create_webhook;


lazy_static! {
    pub static ref AMAZON_REGEX: Regex = Regex::new(
        r"https?://.*?amazon\.co\.jp.*/(gp(/product)?|dp|ASIN)/(?P<asin>[^/?]{10,})\S*"
    ).unwrap();
}


#[derive(Debug, PartialEq)]
pub struct AmazonData {
    price: Option<String>,
    image_url: Option<String>,
    rating: Option<String>,
    title: Option<String>,
    description: Option<String>,
    url: String
}

impl AmazonData {
    pub fn make_embed(&self) -> CreateEmbed {
        CreateEmbed::new()
            .footer(CreateEmbedFooter::new("Created by @猫の手"))
            .url(&self.url)
            .title(self.title.as_deref().unwrap_or("???"))
            .description(self.description.as_deref().unwrap_or("???"))
            .field("評価", self.rating.as_deref().unwrap_or("???"), true)
            .field("値段", self.price.as_deref().unwrap_or("???"), true)
            .thumbnail(self.image_url.as_deref().unwrap_or(""))
    }
}

fn parse_price(element: Option<ElementRef>) -> Option<String> {
    match element {
        Some(price) => {
            Some(
                price
                    .text()
                    .next()
                    .unwrap()
                    .to_string()
                    .replace("\n", "")
            )
        }
        None => None
    }
}

fn parse_image(element: Option<ElementRef>) -> Option<String> {
    match element {
        Some(image) => {
            Some(image.value().attr("src").unwrap().to_string())
        }
        None => None
    }
}

fn parse_rating(element: Option<ElementRef>) -> Option<String> {
    match element {
        Some(rating) => {
            Some(rating.text().next().unwrap().to_string())
        }
        None => None
    }
}

fn parse_title(element: Option<ElementRef>) -> Option<String> {
    match element {
        Some(title) => {
            Some(
                title.value().attr("content").unwrap().to_string()
            )
        }
        None => None
    }
}

fn parse_description(element: Option<ElementRef>) -> Option<String> {
    match element {
        Some(description) => {
            Some(
                description.value().attr("content").unwrap().to_string()
            )
        }
        None => None
    }
}


fn get_data_by_html(html: &str, url: &str) -> AmazonData {
    let price_selector: Selector = Selector::parse(
        "#price,#newBuyBoxPrice,#priceblock_ourprice,#kindle-price,#price_inside_buybox,.slot-price>.a-color-price"
    ).unwrap();
    let image_url_selector: Selector = Selector::parse(
        "#landingImage,#imgBlkFront,#ebooksImgBlkFront"
    ).unwrap();
    let rating_selector: Selector = Selector::parse(
        "span[data-hook=\"rating-out-of-text\"]"
    ).unwrap();
    let title_selector = Selector::parse(
        "meta[name=\"title\"]"
    ).unwrap();
    let description_selector = Selector::parse(
        "meta[name=\"description\"]"
    ).unwrap();

    let document = Html::parse_document(html);
    let price = document.select(&price_selector).next();
    let image_url = document.select(&image_url_selector).next();
    let rating = document.select(&rating_selector).next();
    let title = document.select(&title_selector).next();
    let description = document.select(&description_selector).next();

    AmazonData {
        price: parse_price(price),
        image_url: parse_image(image_url),
        rating: parse_rating(rating),
        title: parse_title(title),
        description: parse_description(description),
        url: url.to_string()
    }
}


pub async fn fetch_amazon_data(url: &str) -> Option<AmazonData> {
    let text = reqwest::get(url)
        .await
        .ok()?
        .text().
        await;
    match text {
        Ok(html) => {
            Some(get_data_by_html(&*html, url))
        },
        Err(_) => {
            None
        }
    }
}


async fn find_amazon_urls(message: &Message) -> Vec<CreateEmbed> {
    let mut amazon_data_list = Vec::new();
    for cap in AMAZON_REGEX.captures_iter(&*message.content) {
        let asin = cap.name("asin").unwrap().as_str();
        let data = fetch_amazon_data(format!("https://www.amazon.co.jp/gp/product/{}", asin).as_str()).await.unwrap();
        amazon_data_list.push(data.make_embed());
    };
    amazon_data_list
}

pub async fn send_amazon_embeds(ctx: &Context, message: &Message) -> Result<(), ()> {
    if let Channel::Guild(channel) = message.channel_id.to_channel(&ctx).await.unwrap() {
        if channel.thread_metadata.is_some() {
            return Err(())
        }
        let data = find_amazon_urls(message).await;
        let webhook = get_or_create_webhook(&ctx, &message).await;
        let content = AMAZON_REGEX.replace_all(&message.content, "https://www.amazon.co.jp/gp/product/$asin").to_string();
        return match webhook {
            Some(webhook) => {
                let result = webhook.execute(&ctx.http, false,
                ExecuteWebhook::new()
                    .embeds(data).content(content)
                    .avatar_url(message.author.avatar_url().unwrap_or(message.author.default_avatar_url().clone()))
                    .username(format!("{}#{}", message.author.name, message.author.discriminator))
                ).await;
                match result {
                    Ok(_) => { Ok(()) }
                    Err(_) => { Err(()) }
                }
            },
            _ => {
                println!("permission error");
                Err(())
            }
        }
    }
    Err(())
}


#[cfg(test)]
mod tests {
    use super::{fetch_amazon_data, AmazonData};

    #[tokio::test]
    async fn fetch_test_amazon_data() {
        let test_data = AmazonData {
            price: Some(" ￥880 ".to_string()),
            image_url: Some("https://m.media-amazon.com/images/I/51oc7UqeIPL._SY346_.jpg".to_string()),
            rating: Some("星5つ中の4.5".to_string()),
            title: Some("日本本土決戦～昭和２０年１１月、米軍皇土へ侵攻す！～ (光文社文庫) | 檜山 良昭 | 日本の小説・文芸 | Kindleストア | Amazon".to_string()),
            description: Some("Amazonで檜山 良昭の日本本土決戦～昭和２０年１１月、米軍皇土へ侵攻す！～ (光文社文庫)。アマゾンならポイント還元本が多数。一度購入いただいた電子書籍は、KindleおよびFire端末、スマートフォンやタブレットなど、様々な端末でもお楽しみいただけます。".to_string()),
            url: "https://www.amazon.co.jp/gp/product/B016K1K0AW/".to_string()
        };
        let data = fetch_amazon_data(
            "https://www.amazon.co.jp/gp/product/B016K1K0AW/"
        ).await;
        assert_eq!(
            data.unwrap(),
            test_data
        )
    }
}
