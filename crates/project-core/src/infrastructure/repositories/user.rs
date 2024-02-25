use crate::prelude::*;
use crate::domain::models::{CreateUserModel, UserModel};
use crate::domain::repositories::UserRepository;
use crate::infrastructure::models::UserSqlModel;
use sqlx::PgPool;

pub struct UserRepositoryImpl {
    db: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: CreateUserModel) -> Result<UserModel> {
        let query = sqlx::query_as!(
            UserSqlModel,
            // language=postgresql
            r#"
            INSERT INTO users (username, email, protected_password)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user.username,
            user.email,
            user.password, // TODO: hash password
        );

        let user = query.fetch_one(&self.db).await?;
        Ok(user.into())
    }
    
    async fn find_by_username(&self, username: String) -> Result<Option<UserModel>> {
        let query = sqlx::query_as!(
            UserSqlModel,
            // language=postgresql
            r#"
            SELECT * FROM users
            WHERE username = $1
            LIMIT 1
            "#,
            username,
        );

        let user = query.fetch_optional(&self.db).await?;
        Ok(user.map(|u| u.into()))
    }
}