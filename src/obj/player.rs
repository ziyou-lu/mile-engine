use crate::obj::Obj;

pub struct Player {

}

impl Obj for Player {
    fn save_data(&self)-> anyhow::Result<anyhow::Error> {
        todo!()
    }
}