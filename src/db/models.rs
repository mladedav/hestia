use serde::{Deserialize, Serialize, Deserializer};

use super::schema::*;

#[derive(Debug, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: Vec<u8>,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub ingredients: Option<String>,
    pub tips: Option<String>,
    pub picture: Option<String>,
    pub preparation_minutes: Option<i32>,
    pub stars: i32,
    pub class: Option<String>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = recipes)]
pub struct NewRecipe {
    pub title: String,
    pub content: Option<String>,
    pub ingredients: Option<String>,
    pub tips: Option<String>,
    pub picture: Option<String>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub preparation_minutes: Option<i32>,
    #[serde(deserialize_with="deserialize_or_default")]
    pub stars: i32,
    pub class: Option<String>,
}

fn deserialize_or_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer).unwrap_or_default();
    Ok(opt.unwrap_or_default())
}
