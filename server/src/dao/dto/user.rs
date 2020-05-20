use crate::dao::schema::users;
use diesel::{Identifiable, Insertable, Queryable};

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq, Eq)]
#[primary_key(id)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}
