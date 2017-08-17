extern crate serde;

macro_rules! min_attribute_type {
    ($($name:ident,)*) => {
        $(
            #[derive(Debug, Serialize, Deserialize)]
            pub struct $name {
                pub href: String,
                pub name: String,
            }
        )*
    }
}

min_attribute_type! {
    Faction,
    Group,
    Rarity,
    CardLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub href: String,
    pub name: String,
    // WARNING: shouldn't be an Option but sometimes the field is missing
    pub uuid: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariationDescriptor {
    pub availability: String,
    pub href: String,
    pub rarity: Rarity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variation {
    pub art: Art,
    pub availability: String,
    pub craft: Cost,
    pub href: String,
    pub mill: Cost,
    pub rarity: Rarity,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Art {
    pub artist: String,
    #[serde(rename = "fullsizeImage")]
    pub fullsize_image: Option<String>,
    #[serde(rename = "thumbnailImage")]
    pub thumbnail_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cost {
    pub normal: i64,
    pub premium: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub categories: Option<Vec<Category>>,
    pub faction: Faction,
    pub flavor: String,
    pub group: Group,
    pub href: String,
    pub info: String,
    pub name: String,
    pub positions: Vec<String>,
    pub strength: i64,
    pub uuid: String,
    pub variations: Vec<VariationDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardPage {
    pub count: i64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<CardLink>,
}
