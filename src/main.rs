use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Deserializer};
use serde_json::json;

fn de_number_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber<T> {
        String(String),
        Number(T),
    }

    match StringOrNumber::<T>::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}

#[derive(Debug, Deserialize)]
struct Database {
    bitems: BItems,
    modifiers: Modifiers,
    tiers: HashMap<String, HashMap<String, Vec<Tier>>>,
    basemods: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Tier {
    #[serde(deserialize_with = "de_number_from_string")]
    ilvl: u16,
    #[serde(deserialize_with = "de_number_from_string")]
    weighting: u16,
    nvalues: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BItems {
    seq: Vec<BaseItem>,
}

#[derive(Debug, Deserialize)]
struct Modifiers {
    seq: Vec<Modifier>,
}

#[derive(Debug, Deserialize)]
struct Modifier {
    name_modifier: String,
    #[serde(deserialize_with = "de_number_from_string")]
    id_modifier: u16,
    modgroups: String,
    affix: String,
}

#[derive(Debug, Deserialize)]
struct BaseItem {
    #[serde(deserialize_with = "de_number_from_string")]
    id_bitem: u16,
    #[serde(deserialize_with = "de_number_from_string")]
    id_base: u16,

    name_bitem: String,

    #[serde(deserialize_with = "de_number_from_string")]
    drop_level: u8,
}

fn main() {
    let file = File::open("formatted_poe.json").unwrap();

    let db: Database = serde_json::from_reader(file).unwrap();

    let vile_robe = db
        .bitems
        .seq
        .iter()
        .find(|b| b.name_bitem == "Vile Robe")
        .unwrap();

    let vile_robe_modifiers = db
        .basemods
        .get(&vile_robe.id_base.to_string())
        .unwrap()
        .iter()
        .map(|mid| {
            let modifier = db
                .modifiers
                .seq
                .iter()
                .find(|m| m.id_modifier.to_string() == *mid)
                .unwrap();

            let tiers = db
                .tiers
                .get(mid)
                .unwrap()
                .get(&vile_robe.id_base.to_string())
                .unwrap();

            let (total_count, max_ilvl, total_weight) = tiers.iter().fold(
                (0, 0u16, 0u32), // Initial state: (count, max_ilvl, total_weight)
                |(count, max_ilvl, weight_acc), tier| {
                    (
                        count + 1,
                        max_ilvl.max(tier.ilvl),
                        weight_acc + tier.weighting as u32, // Cast to u32 to prevent overflow during summation
                    )
                },
            );

            json!({
                "name": modifier.name_modifier.clone(),
                "affix": modifier.affix.clone(),
                "tiers": total_count,
                "ilvl": max_ilvl,
                "weigts": total_weight,
            })
        })
        .collect::<Vec<_>>();

    println!("{:#?}", vile_robe);

    println!("{:#?}", vile_robe_modifiers);
}
