use super::{item::Item, npc::Npc, player::Player};

pub enum ObjType {
    Player(Player),
    Npc(Npc),
    Item(Item),
}