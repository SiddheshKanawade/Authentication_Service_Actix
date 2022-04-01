use uuid::Uuid;
use chrono::NaiveDateTime;
// we use serde to make it more comfortable and compatible with json file
use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Serialize, sqlx::FromRow)]
pub struct User{

    pub id: Uuid,
    pub username: String,
    pub email: String,
    // dont show password, use serde
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime

}

#[derive(Deserialize, Validate)]
pub struct NewUser{
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String
}

#[derive(Deserialize, Validate)]
pub struct UpdateProfile{
    pub full_name: Option<String>,
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>
}