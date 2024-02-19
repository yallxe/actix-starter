use sqlx::PgPool;
use project_core_domain::models::{CreateUserModel, UserModel};
use project_core_domain::repositories::UserRepository;
use project_core_domain::result::DomainResult;
use crate::infrastructure::models::UserSqlModel;

pub struct UserRepositoryImpl {
    db: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: CreateUserModel) -> DomainResult<UserModel> {
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
}