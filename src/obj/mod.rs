use anyhow::{Error, Result};

pub mod player;
pub mod npc;
pub mod item;
pub mod data;
pub mod obj_type;

pub trait Obj {
    fn save_data(&self)-> Result<Error>;
}

