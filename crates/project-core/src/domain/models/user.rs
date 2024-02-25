#[derive(Debug)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    // pub protected_password: String, // i don't think this should be exposed here
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct CreateUserModel {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct RegisterUserModel {
    pub username: String,
    pub email: String,
    pub password: String,
}