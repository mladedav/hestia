use rocket::{fs::TempFile, FromForm};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::schema::*;

#[derive(Debug, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: Vec<u8>,
}

// #[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[derive(Debug, Queryable, Insertable, Identifiable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = recipes)]
pub struct RecipeDb {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub ingredients: Option<String>,
    pub tips: Option<String>,
    pub picture: Option<String>,
    pub preparation_minutes: Option<i32>,
    pub stars: i32,
    pub class: Option<String>,
    pub tags: Option<String>,
}

#[derive(FromForm)]
pub struct RecipeForm<'a> {
    pub title: String,
    pub content: Option<String>,
    pub ingredients: Option<String>,
    pub tips: Option<String>,
    pub picture: Option<TempFile<'a>>,
    pub old_picture: Option<String>,
    pub preparation_minutes: Option<i32>,
    pub stars: i32,
    pub class: Option<String>,
    pub tags: Option<String>,
}

impl<'a> RecipeForm<'a> {
    pub async fn as_new_db(&mut self) -> NewRecipeDb {
        let picture = self.persist_picture().await;

        NewRecipeDb {
            title: self.title.clone(),
            content: self.content.clone(),
            ingredients: self.ingredients.clone(),
            tips: self.tips.clone(),
            picture,
            preparation_minutes: self.preparation_minutes,
            stars: self.stars,
            class: self.class.clone(),
            tags: self.tags.clone(),
        }
    }

    pub async fn as_db(&mut self, id: i32) -> RecipeDb {
        log::info!("Picture: {:?}", self.picture);
        let picture = self
            .persist_picture()
            .await
            .or_else(|| self.old_picture.clone());

        RecipeDb {
            id,
            title: self.title.clone(),
            content: self.content.clone(),
            ingredients: self.ingredients.clone(),
            tips: self.tips.clone(),
            picture,
            preparation_minutes: self.preparation_minutes,
            stars: self.stars,
            class: self.class.clone(),
            tags: self.tags.clone(),
        }
    }

    async fn persist_picture(&mut self) -> Option<String> {
        match &mut self.picture {
            Some(file) if file.len() > 0 => {
                log::info!("Persisting picture");
                let extension = file
                    .content_type()
                    .unwrap()
                    .extension()
                    .map_or("unknown", |x| x.as_str());
                let name = format!("{}.{}", Uuid::new_v4(), extension);
                let res = file.persist_to(format!("./pictures/{}", name)).await;
                log::info!("Picture persistence result: {:?}", res);
                Some(name)
            }
            _ => None,
        }
    }
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = recipes)]
pub struct NewRecipeDb {
    pub title: String,
    pub content: Option<String>,
    pub ingredients: Option<String>,
    pub tips: Option<String>,
    pub picture: Option<String>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub preparation_minutes: Option<i32>,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub stars: i32,
    pub class: Option<String>,
    pub tags: Option<String>,
}

fn deserialize_or_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer).unwrap_or_default();
    Ok(opt.unwrap_or_default())
}
