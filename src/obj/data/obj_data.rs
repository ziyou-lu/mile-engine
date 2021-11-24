use std::{collections::HashMap};
use crate::{MILE, obj::obj_type::ObjType};

use super::data_type::DataType;

pub struct ObjData {
    pub props: HashMap<String, DataType>,
    pub temps: HashMap<String, DataType>,
    pub recs: HashMap<String, Rec>,
    pub boxes: HashMap<String, Vec<ObjType>>,

}

pub struct Rec {
    pub schema: Vec<DataType>,
    pub data: Vec<Vec<DataType>>,
}

impl ObjData {
    pub fn set_prop(&mut self, name: String, prop: DataType) {
        if self.props.contains_key(&name) {
            self.props.insert(name, prop);
        } else {
            MILE.get_context().log.error(format!("设置属性错误，不存在该属性！"));
        }
    }

    pub fn get_prop(&mut self, name: String) -> Option<&DataType> {
        if self.props.contains_key(&name) {
            return self.props.get(&name);
        } 
        
        None
    }

    pub fn set_temp(&mut self, name: String, temp: DataType) {
        if self.temps.contains_key(&name) {
            self.temps.insert(name, temp);
        } else {
            MILE.get_context().log.error(format!("设置临时属性错误，不存在该属性！"));
        }
    }

    pub fn get_temp(&mut self, name: String) -> Option<&DataType> {
        if self.temps.contains_key(&name) {
            return self.temps.get(&name);
        } 
        
        None
    }

}