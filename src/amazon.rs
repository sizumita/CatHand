use scraper::{Html, Selector, ElementRef};
use scraper::html::Select;


#[derive(Debug, PartialEq)]
pub struct AmazonData {
    price: Option<String>,
    image_url: Option<String>,
    rating: Option<String>,
    title: Option<String>,
    description: Option<String>,
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

fn get_data_by_html(html: &str) -> AmazonData {
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
        description: parse_description(description)
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
            Some(get_data_by_html(&*html))
        },
        Err(_) => {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{fetch_amazon_data, AmazonData};

    #[tokio::test]
    async fn fetch_test_amazon_data() {
        let test_data = AmazonData {
            price: Some("￥880".to_string()),
            image_url: Some("https://m.media-amazon.com/images/I/51oc7UqeIPL._SY346_.jpg".to_string()),
            rating: Some("星5つ中の4.4".to_string()),
            title: Some("日本本土決戦～昭和２０年１１月、米軍皇土へ侵攻す！～ (光文社文庫) | 檜山 良昭 | 日本の小説・文芸 | Kindleストア | Amazon".to_string()),
            description: Some("Amazonで檜山 良昭の日本本土決戦～昭和２０年１１月、米軍皇土へ侵攻す！～ (光文社文庫)。アマゾンならポイント還元本が多数。一度購入いただいた電子書籍は、KindleおよびFire端末、スマートフォンやタブレットなど、様々な端末でもお楽しみいただけます。".to_string())
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
