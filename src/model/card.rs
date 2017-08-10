extern crate serde;

#[derive(Debug, Deserialize)]
pub struct Category {
    href: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Faction {
    href: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    href: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Rarity {
    href: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Variation {
    availability: String,
    href: String,
    rarity: Rarity,
}

#[derive(Debug, Deserialize)]
pub struct Card {
    categories: Option<Vec<String>>,
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

#[derive(Debug, Deserialize)]
pub struct CardLink {
    href: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct PageCard {
    count: i64,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<CardLink>,
}
