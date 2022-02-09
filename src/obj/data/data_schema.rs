use std::collections::HashMap;

pub struct PropSchema {
    pub name : String, 
    pub data_type : String,
    pub save : bool,
    pub owner_visible : bool,
    pub other_visible : bool,
}

pub struct RecSchema {
    pub name : String, 
    pub save : bool,
    pub owner_visible : bool,
    pub other_visible : bool,
    pub row_schema : Vec<RowSchema>,
}

pub struct RowSchema {
    pub name : String, 
    pub data_type: String,
}

pub struct GameObjectSchema {
    pub props: HashMap<String, PropSchema>,
    pub temps: HashMap<String, PropSchema>,
    pub recs: HashMap<String, RecSchema>,
    pub boxes: HashMap<String, BoxSchema>,
}

pub struct BoxSchema {
    pub name : String, 
    pub box_type : String,
    pub obj_type : String,
    pub save : bool,
    pub owner_visible : bool,
    pub other_visible : bool,
}