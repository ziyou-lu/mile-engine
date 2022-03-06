use std::collections::HashMap;
use toml;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct PropSchema {
    pub name : String, 
    pub data_type : String,
    pub save : bool,
    pub owner_visible : bool,
    pub other_visible : bool,
}

#[derive(Deserialize)]
pub struct RecSchema {
    pub name : String, 
    pub save : bool,
    pub owner_visible : bool,
    pub other_visible : bool,
    pub row_schema : Vec<RowSchema>,
}

#[derive(Deserialize)]
pub struct RowSchema {
    pub name : String, 
    pub data_type: String,
}

#[derive(Deserialize)]
pub struct GameObjectSchema {
    pub props: HashMap<String, PropSchema>,
    pub temps: HashMap<String, PropSchema>,
    pub recs: HashMap<String, RecSchema>,
    pub boxes: HashMap<String, BoxSchema>,
}

#[derive(Deserialize)]
pub struct BoxSchema {
    pub name : String, 
    pub box_type : String,
    pub obj_type : String,
    pub save : bool,
    pub owner_visible : bool,
    pub other_visible : bool,
}