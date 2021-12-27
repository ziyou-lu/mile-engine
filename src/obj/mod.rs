use anyhow::{Error, Result};

use self::data::obj_data::ObjData;

pub mod data;

pub enum ObjType {
    Npc(ObjData),
    Player(ObjData),
    Item(ObjData),
    Scene(ObjData),
}

const

impl Into<ObjData> for ObjType {
    
    fn into(self) -> ObjData {
        match self {
            ObjType::Npc(o) => return o,
            ObjType::Player(o) => return o,
            ObjType::Item(o) => return o,
            ObjType::Scene(o) => return o,
            _ => panic!("不可识别的对象类型"),
        }
    }
}

pub struct Obj {
    obj_type: ObjType,
}

impl Obj {
    pub fn save_data(&self) -> Result<()> {
        let obj_data: ObjData = self.obj_type.into();
    }
}
