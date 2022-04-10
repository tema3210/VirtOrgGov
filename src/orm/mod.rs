use diesel::{Queryable,QueryableByName};
use uuid::Uuid;

#[derive(Queryable,Clone,Debug)]
pub struct Codex {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable,Clone,Debug)]
pub struct Party {
    pub id: Uuid,
    pub name: String,
}

use crate::schema::law_data;
#[derive(Queryable,QueryableByName,Clone,Debug)]
#[table_name(law_data)]
pub struct LawData {
    // #[sql_type = "diesel::sql_types::Uuid"]
    pub what: Uuid,
    // #[sql_type = "diesel::sql_types::Varchar"]
    pub new_text: String,
    // #[sql_type = "diesel::sql_types::Timestamp"]
    pub submit_date: chrono::NaiveDateTime,
    // #[sql_type = "diesel::sql_types::Varchar"]
    pub status: String,
}

#[derive(Queryable,Clone,Debug)]
pub struct Politic {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub lastname: String,
    pub age: i16,
}

#[derive(Queryable,Clone,Debug)]
pub struct Law {
    pub id: Uuid,
    pub codex: Uuid,
    pub author: Uuid,
}