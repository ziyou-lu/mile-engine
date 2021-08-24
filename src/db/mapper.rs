use rbatis::crud_table;

#[crud_table(table_name:"user")]
pub struct User {
    pub user_name: Option<String>,
    pub password: Option<String>
}