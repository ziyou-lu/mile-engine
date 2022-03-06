use anyhow::{Error, Result};

use self::data::obj_data::GameData;

pub mod data;
pub mod schema;

pub enum ObjType {
    GameObj(GameData),
    GameBox(GameData),
}

impl Into<GameData> for ObjType {
    
    fn into(self) -> GameData {
        match self {
            ObjType::GameObj(o) => return o,
            ObjType::GameBox(o) => return o,
            _ => panic!("不可识别的对象类型"),
        }
    }
}

pub struct Obj {
    obj_type: ObjType,
}

impl Obj {
    /*pub fn save_data(&self) -> Result<()> {
        let obj_data: ObjData = self.obj_type.into();
    }*/
}
