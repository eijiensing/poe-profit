use crate::importer::{Affix, BaseItemId, ModifierDefinition, ModifierId, PoeData, Socketable};

pub struct RolledModifier {
    pub modifier_id: ModifierId,
    pub values: Vec<f32>,
}

pub struct CraftedItem {
    pub item_level: u8,
    pub base_item_id: BaseItemId,
    pub modifiers: Vec<RolledModifier>,
}

pub enum Currency {
    OrbOfTransmutation,
    GreaterOrbOfTransmutation,
    PerfectOrbOfTransmutation,
}

pub enum Omen {}

pub struct CraftingOperation {
    pub currency: Currency,
    pub omens: Vec<Omen>,
}

#[derive(Debug, Clone, Copy)]
pub struct StateModifier {
    pub modifier_id: ModifierId,
    pub tier: u8,
}

#[derive(Debug)]
pub struct ItemState {
    pub item_level: u8,
    pub base_item_id: BaseItemId,
    pub modifiers: Vec<StateModifier>,
    pub sockets: Vec<Option<Socketable>>,
    pub probability: f64,
}

pub fn possible_next_states(
    poe_data: &PoeData,
    start_state: &ItemState,
    crafting_operation: CraftingOperation,
) -> Vec<ItemState> {
    let base_item = poe_data
        .base_items
        .iter()
        .find(|mbi| mbi.id == start_state.base_item_id)
        .unwrap();

    let minimum_modifier_level = match crafting_operation.currency {
        Currency::OrbOfTransmutation => 0,
        Currency::GreaterOrbOfTransmutation => 44,
        Currency::PerfectOrbOfTransmutation => 70,
    };

    let (prefix_count, suffix_count) = start_state.modifiers.iter().fold((0, 0), |acc, m| {
        let md = base_item
            .modifier_definitions
            .iter()
            .find(|md| md.id == m.modifier_id)
            .unwrap();
        if matches!(md.affix, Affix::Prefix) {
            return (acc.0 + 1, acc.1);
        } else if matches!(md.affix, Affix::Prefix) {
            return (acc.0, acc.1 + 1);
        }

        acc
    });

    let possible_modifiers = base_item
        .modifier_definitions
        .iter()
        .flat_map(|md| md.tiers.iter().map(move |t| (md, t)))
        .filter_map(|(md, t)| {
            if t.item_level < minimum_modifier_level || t.item_level > start_state.item_level {
                // TODO: keep atleast one in family (highest tier)
                return None;
            }

            if matches!(md.affix, Affix::Prefix) && prefix_count >= 3
                || matches!(md.affix, Affix::Suffix) && suffix_count >= 3
            {
                return None;
            }

            if start_state
                .modifiers
                .iter()
                .flat_map(|ms| {
                    base_item
                        .modifier_definitions
                        .iter()
                        .find(|ssmd| ssmd.id == ms.modifier_id)
                        .unwrap()
                        .modifier_families
                        .iter()
                })
                .any(|mf| md.modifier_families.contains(mf))
            {
                return None;
            }

            Some((
                t.weighting,
                StateModifier {
                    modifier_id: md.id,
                    tier: t.tier,
                },
            ))
        })
        .collect::<Vec<_>>();

    let total_weight: f64 = possible_modifiers.iter().map(|pm| pm.0 as f64).sum();

    let total_weight_prefix: f64 = possible_modifiers
        .iter()
        .filter_map(|pm| {
            if matches!(
                base_item
                    .modifier_definitions
                    .iter()
                    .find(|md| md.id == pm.1.modifier_id)
                    .unwrap()
                    .affix,
                Affix::Prefix
            ) {
                return Some(pm.0 as f64);
            }
            None
        })
        .sum();
    let total_weight_suffix: f64 = possible_modifiers
        .iter()
        .filter_map(|pm| {
            if matches!(
                base_item
                    .modifier_definitions
                    .iter()
                    .find(|md| md.id == pm.1.modifier_id)
                    .unwrap()
                    .affix,
                Affix::Suffix
            ) {
                return Some(pm.0 as f64);
            }
            None
        })
        .sum();

    println!(
        "total: {total_weight} | prefix: {total_weight_prefix} | suffix: {total_weight_suffix}"
    );

    let possible_next_states = possible_modifiers
        .iter()
        .map(|pm| {
            let mut modifiers = start_state.modifiers.clone();
            modifiers.push(pm.1);
            ItemState {
                item_level: start_state.item_level,
                base_item_id: start_state.base_item_id,
                modifiers,
                sockets: start_state.sockets.clone(),
                probability: pm.0 as f64, //((pm.0 as f64 / total_weight) * 100f64),
            }
        })
        .collect::<Vec<_>>();

    possible_next_states
}
