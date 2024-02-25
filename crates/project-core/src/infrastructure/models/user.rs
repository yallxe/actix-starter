use sqlx::FromRow;
use crate::domain::models::UserModel;

#[derive(FromRow)]
pub struct UserSqlModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub protected_password: String,
    pub created_at: chrono::DateTime<chrono::Utc>
}

impl From<UserSqlModel> for UserModel {
    fn from(value: UserSqlModel) -> Self {
        UserModel {
            id: value.id,
            username: value.username,
            email: value.email,
            created_at: value.created_at,
        }
    }
}
