use crate::{
    crafting_engine::{CraftedItem, RolledModifier},
    importer::{Affix, load_poe_data_cached},
};

mod cli;
mod crafting_engine;
mod importer;

fn main() {
    let poe_data =
        load_poe_data_cached("poec_data.json").expect("Failed to load and parse database");

    let vile_robe = poe_data
        .base_items
        .iter()
        .find(|mbi| mbi.name == "Vile Robe")
        .unwrap();

    let spirit_prefix = vile_robe
        .modifier_definitions
        .iter()
        .find(|md| md.name == "+# to Spirit" && md.affix == Affix::Prefix)
        .unwrap();

    println!("{:#?}", vile_robe);

    let my_vile_robe = CraftedItem {
        item_level: 82,
        base_item_id: vile_robe.id,
        prefixes: vec![RolledModifier {
            modifier_id: spirit_prefix.id,
            values: vec![58f32],
        }],
        suffixes: vec![],
    };

    cli::print_item(&poe_data, &my_vile_robe);
}
