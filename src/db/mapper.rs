use rbatis::crud_table;

#[crud_table(table_name:"user")]
pub struct User {
    pub user_name: String,
    pub password: String
}