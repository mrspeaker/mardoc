use bevy::prelude::*;
use std::collections::HashMap;

/// Type of body piece
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BodyPartType {
    Head,
    Torso,
    Leg,
    Arm,
}

/// Type of item
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Generic,
    BodyPart(BodyPartType),
    Tool { durability: i16 },
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
)]
pub enum ItemId {
    #[default]
    Head,
    Torso,
    Leg,
    Arm,
    Apple
}

impl ItemId {
    pub fn get_default_type(&self) -> ItemType {
        match *self {
            Self::Head => ItemType::BodyPart(BodyPartType::Head),
            Self::Torso => ItemType::BodyPart(BodyPartType::Torso),
            Self::Arm => ItemType::BodyPart(BodyPartType::Arm),
            Self::Leg => ItemType::BodyPart(BodyPartType::Leg),
            Self::Apple => ItemType::Generic
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct ItemStack {
    pub item_id: ItemId,
    pub item_type: ItemType,
    pub num: u32,
}

impl ItemStack {
    pub fn test() -> Self {
        Self {
            item_id: ItemId::Leg,
            item_type: ItemId::Leg.get_default_type(),
            num: 2
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct Inventory {
    pub map: HashMap<u32, ItemStack>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add_item(&mut self, mut stack: ItemStack) {
        for i in 0..10 {
            let item_opt = self.map.get(&i);
            if item_opt.is_some() {
                let slot_stack = item_opt.expect("err empty");
                if slot_stack.item_id != stack.item_id {
                    continue;
                }
                // add it
                stack.num += slot_stack.num
            }

            let ins_stack = ItemStack {
                item_id: stack.item_id,
                item_type: stack.item_type,
                num: stack.num
            };
            self.map.insert(i, ins_stack);
            break;
        }

    }
}
