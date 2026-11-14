use crate::importer::{Affix, BaseItem, BaseItemId, ModifierId, ModifierType, PoeData, Socketable};

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
    OrbOfAugmentation,
    GreaterOrbOfAugmentation,
    PerfectOrbOfAugmentation,
    ExaltedOrb,
    GreaterExaltedOrb,
    PerfectExaltedOrb,
    ChaosOrb,
    GreaterChaosOrb,
    PerfectChaosOrb,
    OrbOfAnnulment,
    VaalOrb,
}

impl Currency {
    pub fn minimum_modifier_level(&self) -> u8 {
        match self {
            Currency::OrbOfTransmutation => 0,
            Currency::GreaterOrbOfTransmutation => 44,
            Currency::PerfectOrbOfTransmutation => 70,
            Currency::OrbOfAugmentation => 0,
            Currency::GreaterOrbOfAugmentation => 44,
            Currency::PerfectOrbOfAugmentation => 70,
            Currency::ExaltedOrb => 0,
            Currency::GreaterExaltedOrb => 35,
            Currency::PerfectExaltedOrb => 50,
            Currency::ChaosOrb => 0,
            Currency::GreaterChaosOrb => 35,
            Currency::PerfectChaosOrb => 50,
            Currency::OrbOfAnnulment => 0,
            Currency::VaalOrb => 0,
        }
    }

    fn forward(
        &self,
        poe_data: &PoeData,
        item: &ItemState,
        context: &CraftingContext,
    ) -> Vec<ItemState> {
        match self {
            Currency::OrbOfTransmutation
            | Currency::GreaterOrbOfTransmutation
            | Currency::PerfectOrbOfTransmutation
            | Currency::OrbOfAugmentation
            | Currency::GreaterOrbOfAugmentation
            | Currency::PerfectOrbOfAugmentation
            | Currency::ExaltedOrb
            | Currency::GreaterExaltedOrb
            | Currency::PerfectExaltedOrb => {
                let base_item = poe_data
                    .base_items
                    .iter()
                    .find(|mbi| mbi.id == item.base_item_id)
                    .unwrap();

                add_modifier_states(base_item, item, self.minimum_modifier_level())
            }
            Currency::ChaosOrb | Currency::GreaterChaosOrb | Currency::PerfectChaosOrb => {
                let base_item = poe_data
                    .base_items
                    .iter()
                    .find(|mbi| mbi.id == item.base_item_id)
                    .unwrap();
                let minimum_modifier_level = self.minimum_modifier_level();

                let mut possible_next_states = Vec::new();
                for removed_state in remove_modifier_states(item) {
                    // Removal probabilities are percentages, while the addition
                    // probabilities from add_modifier_states are conditional percentages.
                    let removal_probability = removed_state.probability / 100.0;

                    for mut next_state in
                        add_modifier_states(base_item, &removed_state, minimum_modifier_level)
                    {
                        next_state.probability *= removal_probability;
                        possible_next_states.push(next_state);
                    }
                }

                possible_next_states
            }
            Currency::OrbOfAnnulment => remove_modifier_states(item),
            Currency::VaalOrb => todo!(),
        }
    }

    fn backward(
        &self,
        poe_data: &PoeData,
        target: &ItemState,
        context: &CraftingContext,
    ) -> Vec<ItemState> {
        match self {
            Currency::OrbOfTransmutation => todo!(),
            Currency::GreaterOrbOfTransmutation => todo!(),
            Currency::PerfectOrbOfTransmutation => todo!(),
            Currency::OrbOfAugmentation => todo!(),
            Currency::GreaterOrbOfAugmentation => todo!(),
            Currency::PerfectOrbOfAugmentation => todo!(),
            Currency::ExaltedOrb => todo!(),
            Currency::GreaterExaltedOrb => todo!(),
            Currency::PerfectExaltedOrb => todo!(),
            Currency::ChaosOrb => todo!(),
            Currency::GreaterChaosOrb => todo!(),
            Currency::PerfectChaosOrb => todo!(),
            Currency::OrbOfAnnulment => todo!(),
            Currency::VaalOrb => todo!(),
        }
    }
}

/// Removes each explicit modifier once, producing one possible state per modifier.
fn remove_modifier_states(item: &ItemState) -> Vec<ItemState> {
    let removal_count = item.modifiers.len();
    if removal_count == 0 {
        return Vec::new();
    }

    let probability = 100.0 / removal_count as f64;

    item.modifiers
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let mut modifiers = item.modifiers.clone();
            modifiers.remove(index);
            ItemState {
                item_level: item.item_level,
                base_item_id: item.base_item_id,
                modifiers,
                sockets: item.sockets.clone(),
                probability,
            }
        })
        .collect()
}

/// Adds one eligible explicit modifier, producing one state per modifier tier.
fn add_modifier_states(
    base_item: &BaseItem,
    item: &ItemState,
    minimum_modifier_level: u8,
) -> Vec<ItemState> {
    let (prefix_count, suffix_count) = item.prefix_suffix_count(base_item);

    let possible_modifiers = base_item
        .modifier_definitions
        .iter()
        .filter(|md| {
            matches!(md.modifier_type, ModifierType::Base)
                && matches!(md.affix, Affix::Prefix | Affix::Suffix)
        })
        .flat_map(|md| {
            let mut eligible_tiers: Vec<_> = md
                .tiers
                .iter()
                .filter(|t| {
                    t.item_level >= minimum_modifier_level && t.item_level <= item.item_level
                })
                .collect();

            // Keep at least one tier per modifier family: if all tiers were
            // filtered out, fall back to the highest available tier
            if eligible_tiers.is_empty() {
                if let Some(fallback) = md
                    .tiers
                    .iter()
                    .filter(|t| t.item_level <= item.item_level)
                    .min_by_key(|t| t.tier)
                {
                    eligible_tiers = vec![fallback];
                }
            }

            eligible_tiers.into_iter().map(move |t| (md, t))
        })
        .filter_map(|(md, t)| {
            if matches!(md.affix, Affix::Prefix) && prefix_count >= 3
                || matches!(md.affix, Affix::Suffix) && suffix_count >= 3
            {
                return None;
            }

            if item
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
    if total_weight <= 0.0 {
        return Vec::new();
    }

    possible_modifiers
        .iter()
        .map(|pm| {
            let mut modifiers = item.modifiers.clone();
            modifiers.push(pm.1);
            ItemState {
                item_level: item.item_level,
                base_item_id: item.base_item_id,
                modifiers,
                sockets: item.sockets.clone(),
                probability: (pm.0 as f64 / total_weight) * 100f64,
            }
        })
        .collect()
}

pub enum CraftingAtom {
    AddModifier,
    RemoveModifier,
    RerollStatsWithinTier,
    AddImplicit,
    AddSocket,
    AddCorruptedImplicit,
    CorruptItem,
}

// Context of one crafting action
pub struct CraftingContext {
    maybe_omen: Option<Omen>,
}

pub enum Omen {}

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

impl ItemState {
    pub fn prefix_suffix_count(&self, base_item: &BaseItem) -> (u8, u8) {
        self.modifiers.iter().fold((0, 0), |acc, m| {
            let md = base_item
                .modifier_definitions
                .iter()
                .find(|md| md.id == m.modifier_id)
                .unwrap();
            if matches!(md.affix, Affix::Prefix) {
                return (acc.0 + 1, acc.1);
            } else if matches!(md.affix, Affix::Suffix) {
                return (acc.0, acc.1 + 1);
            }

            acc
        })
    }
}

// pub fn possible_next_states(
//     poe_data: &PoeData,
//     start_state: &ItemState,
//     crafting_step: CraftingStep,
// ) -> Vec<ItemState> {
//     let base_item = poe_data
//         .base_items
//         .iter()
//         .find(|mbi| mbi.id == start_state.base_item_id)
//         .unwrap();
//
//     let minimum_modifier_level = match crafting_step.currency {
//         Currency::OrbOfTransmutation => 0,
//         Currency::GreaterOrbOfTransmutation => 44,
//         Currency::PerfectOrbOfTransmutation => 70,
//         Currency::OrbOfAugmentation => 0,
//         Currency::GreaterOrbOfAugmentation => 44,
//         Currency::PerfectOrbOfAugmentation => 70,
//         Currency::ExaltedOrb => 0,
//         Currency::GreaterExaltedOrb => 35,
//         Currency::PerfectExaltedOrb => 50,
//     };
//
//     let (prefix_count, suffix_count) = start_state.modifiers.iter().fold((0, 0), |acc, m| {
//         let md = base_item
//             .modifier_definitions
//             .iter()
//             .find(|md| md.id == m.modifier_id)
//             .unwrap();
//         if matches!(md.affix, Affix::Prefix) {
//             return (acc.0 + 1, acc.1);
//         } else if matches!(md.affix, Affix::Suffix) {
//             return (acc.0, acc.1 + 1);
//         }
//
//         acc
//     });
//
//     let possible_modifiers = base_item
//         .modifier_definitions
//         .iter()
//         .filter(|md| matches!(md.modifier_type, ModifierType::Base))
//         .filter(|md| matches!(md.affix, Affix::Prefix | Affix::Suffix))
//         .flat_map(|md| {
//             let mut eligible_tiers: Vec<_> = md
//                 .tiers
//                 .iter()
//                 .filter(|t| {
//                     t.item_level >= minimum_modifier_level && t.item_level <= start_state.item_level
//                 })
//                 .collect();
//
//             // Keep at least one tier per modifier family: if all tiers were
//             // filtered out, fall back to the highest available tier
//             if eligible_tiers.is_empty() {
//                 if let Some(fallback) = md
//                     .tiers
//                     .iter()
//                     .filter(|t| t.item_level <= start_state.item_level)
//                     .min_by_key(|t| t.tier)
//                 {
//                     eligible_tiers = vec![fallback];
//                 }
//             }
//
//             eligible_tiers.into_iter().map(move |t| (md, t))
//         })
//         .filter_map(|(md, t)| {
//             if matches!(md.affix, Affix::Prefix) && prefix_count >= 3
//                 || matches!(md.affix, Affix::Suffix) && suffix_count >= 3
//             {
//                 return None;
//             }
//
//             if start_state
//                 .modifiers
//                 .iter()
//                 .flat_map(|ms| {
//                     base_item
//                         .modifier_definitions
//                         .iter()
//                         .find(|ssmd| ssmd.id == ms.modifier_id)
//                         .unwrap()
//                         .modifier_families
//                         .iter()
//                 })
//                 .any(|mf| md.modifier_families.contains(mf))
//             {
//                 return None;
//             }
//
//             Some((
//                 t.weighting,
//                 StateModifier {
//                     modifier_id: md.id,
//                     tier: t.tier,
//                 },
//             ))
//         })
//         .collect::<Vec<_>>();
//
//     let total_weight: f64 = possible_modifiers.iter().map(|pm| pm.0 as f64).sum();
//
//     let total_weight_prefix: f64 = possible_modifiers
//         .iter()
//         .filter_map(|pm| {
//             if matches!(
//                 base_item
//                     .modifier_definitions
//                     .iter()
//                     .find(|md| md.id == pm.1.modifier_id)
//                     .unwrap()
//                     .affix,
//                 Affix::Prefix
//             ) {
//                 return Some(pm.0 as f64);
//             }
//             None
//         })
//         .sum();
//     let total_weight_suffix: f64 = possible_modifiers
//         .iter()
//         .filter_map(|pm| {
//             if matches!(
//                 base_item
//                     .modifier_definitions
//                     .iter()
//                     .find(|md| md.id == pm.1.modifier_id)
//                     .unwrap()
//                     .affix,
//                 Affix::Suffix
//             ) {
//                 return Some(pm.0 as f64);
//             }
//             None
//         })
//         .sum();
//
//     println!(
//         "total: {total_weight} | prefix: {total_weight_prefix} | suffix: {total_weight_suffix}"
//     );
//
//     let possible_next_states = possible_modifiers
//         .iter()
//         .map(|pm| {
//             let mut modifiers = start_state.modifiers.clone();
//             modifiers.push(pm.1);
//             ItemState {
//                 item_level: start_state.item_level,
//                 base_item_id: start_state.base_item_id,
//                 modifiers,
//                 sockets: start_state.sockets.clone(),
//                 probability: pm.0 as f64, //((pm.0 as f64 / total_weight) * 100f64),
//             }
//         })
//         .collect::<Vec<_>>();
//
//     possible_next_states
// }
