use crate::importer::{BaseItemId, ModifierId};

pub struct RolledModifier {
    pub modifier_id: ModifierId,
    pub values: Vec<f32>,
}

pub struct CraftedItem {
    pub item_level: u8,
    pub base_item_id: BaseItemId,
    pub prefixes: Vec<RolledModifier>,
    pub suffixes: Vec<RolledModifier>,
}
