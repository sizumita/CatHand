use encoding_rs::EUC_JP;
extern crate hex;

pub trait Decoder {
    fn decode(content: &str) -> String;
}

pub struct Utf8Decoder;

impl Decoder for Utf8Decoder {
    fn decode(content: &str) -> String {
        return content.to_string()
    }
}

pub struct EucJpDecoder;

impl Decoder for EucJpDecoder {
    fn decode(content: &str) -> String {
        let bytes = &hex::decode(content.replace('%', "")).unwrap()[..];
        let (cow, _encoding_used, _had_errors) = EUC_JP.decode(bytes);
        cow.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::{Decoder, EucJpDecoder};

    #[test]
    fn test_euc_jp() {
        assert_eq!(EucJpDecoder::decode(&*"%C1%E0%BA%EE%CA%FD%CB%A1"), "操作方法")
    }
}
