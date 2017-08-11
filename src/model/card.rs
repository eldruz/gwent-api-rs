extern crate serde;

macro_rules! min_attribute_type {
    ($($name:ident,)*) => {
        $(
            #[derive(Debug, Serialize, Deserialize)]
            pub struct $name {
                href: String,
                name: String,
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
    href: String,
    name: String,
    // WARNING: shouldn't be an Option but sometimes the field is missing
    uuid: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variation {
    availability: String,
    href: String,
    rarity: Rarity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    categories: Option<Vec<Category>>,
    faction: Faction,
    flavor: String,
    group: Group,
    href: String,
    info: String,
    name: String,
    positions: Vec<String>,
    strength: i64,
    uuid: String,
    variations: Vec<Variation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardPage {
    count: i64,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<CardLink>,
}
