use crate::{
    crafting_engine::{
        CraftedItem, CraftingOperation, Currency, ItemState, RolledModifier, possible_next_states,
    },
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

    let my_vile_robe = CraftedItem {
        item_level: 82,
        base_item_id: vile_robe.id,
        modifiers: vec![RolledModifier {
            modifier_id: spirit_prefix.id,
            values: vec![58f32],
        }],
    };

    cli::print_item(&poe_data, &my_vile_robe);

    let next_states = possible_next_states(
        &poe_data,
        &ItemState {
            item_level: 82,
            base_item_id: vile_robe.id,
            modifiers: vec![],
            sockets: vec![],
            probability: 100f64,
        },
        CraftingOperation {
            currency: Currency::PerfectOrbOfTransmutation,
            omens: vec![],
        },
    );

    for ns in next_states {
        println!(
            "{:.2}% | Tier {} | {}",
            ns.probability,
            ns.modifiers.first().unwrap().tier,
            vile_robe
                .modifier_definitions
                .iter()
                .find(|md| md.id == ns.modifiers.first().unwrap().modifier_id)
                .unwrap()
                .name
                .clone(),
        )
    }
}
