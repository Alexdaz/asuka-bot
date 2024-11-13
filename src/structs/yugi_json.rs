use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub frame_type: String,
    pub desc: String,
    pub atk: Option<i16>,
    pub def: Option<i16>,
    pub level: Option<u8>,
    pub race: String,
    pub attribute: Option<String>,
    pub archetype: Option<String>,
    #[serde(rename = "ygoprodeck_url")]
    pub ygoprodeck_url: String,
    #[serde(rename = "card_sets")]
    pub card_sets: Vec<CardSet>,
    #[serde(rename = "card_images")]
    pub card_images: Vec<CardImage>,
    #[serde(rename = "card_prices")]
    pub card_prices: Vec<CardPrice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardSet {
    #[serde(rename = "set_name")]
    pub set_name: String,
    #[serde(rename = "set_code")]
    pub set_code: String,
    #[serde(rename = "set_rarity")]
    pub set_rarity: String,
    #[serde(rename = "set_rarity_code")]
    pub set_rarity_code: String,
    #[serde(rename = "set_price")]
    pub set_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardImage {
    pub id: i64,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "image_url_small")]
    pub image_url_small: String,
    #[serde(rename = "image_url_cropped")]
    pub image_url_cropped: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPrice {
    #[serde(rename = "cardmarket_price")]
    pub cardmarket_price: String,
    #[serde(rename = "tcgplayer_price")]
    pub tcgplayer_price: String,
    #[serde(rename = "ebay_price")]
    pub ebay_price: String,
    #[serde(rename = "amazon_price")]
    pub amazon_price: String,
    #[serde(rename = "coolstuffinc_price")]
    pub coolstuffinc_price: String,
}
