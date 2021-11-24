use crate::obj::Obj;

use super::data::obj_data::ObjData;

pub struct Npc {
    data: ObjData,
}

impl Obj for Npc {
    fn save_data(&self)-> anyhow::Result<anyhow::Error> {
        todo!()
    }
}