use crate::{
    crafting_engine::CraftedItem,
    importer::{Affix, PoeData, RollableValue},
};

pub fn print_item(poe_data: &PoeData, crafted_item: &CraftedItem) {
    let base_item = poe_data
        .base_items
        .iter()
        .find(|bi| bi.id == crafted_item.base_item_id)
        .unwrap();

    let width = base_item.name.len() + 30;

    let modifier_texts = crafted_item
        .modifiers
        .iter()
        .map(|rm| {
            let modifier_definition = base_item
                .modifier_definitions
                .iter()
                .find(|md| md.id == rm.modifier_id)
                .unwrap();

            let tier = modifier_definition
                .tiers
                .iter()
                .find(|t| {
                    t.values.iter().enumerate().all(|(i, vd)| {
                        let rolled_value = rm.values.get(i).expect("Rolled value to exist");
                        match vd {
                            RollableValue::Between((min, max)) => {
                                rolled_value >= min && rolled_value <= max
                            }
                            RollableValue::Fixed(value) => value == rolled_value,
                        }
                    })
                })
                .unwrap();

            rolled_modifier_text(
                &modifier_definition.name,
                &rm.values,
                &modifier_definition.affix,
                tier.tier,
            )
        })
        .collect::<Vec<_>>();

    let item_level_text = format!("Item level: {}", crafted_item.item_level);

    println!("╔{:═^width$}╗", "");
    println!("║{:^width$}║", base_item.name);
    println!("╠{:═^width$}╣", "");
    println!("║{:^width$}║", item_level_text);
    println!("╠{:═^width$}╣", "");

    for modifier_text in modifier_texts {
        println!("║{:^width$}║", modifier_text);
    }

    println!("╚{:═^width$}╝", "");
}

fn rolled_modifier_text(
    modifier_text: &String,
    values: &Vec<f32>,
    affix: &Affix,
    tier: u8,
) -> String {
    let affix_letter = match affix {
        Affix::Prefix => "P",
        Affix::Suffix => "S",
        _ => "",
    };

    let mut modifier_text = format!("({affix_letter}{tier}) {modifier_text}");
    for v in values.iter() {
        modifier_text = modifier_text.as_str().replacen("#", &v.to_string(), 1);
    }
    modifier_text
}
