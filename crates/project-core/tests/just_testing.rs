use sqlx::PgPool;
use project_core::infrastructure::repositories::UserRepositoryImpl;
use project_core::services::RegistrationServiceImpl;
use project_core_domain::models::RegisterUserModel;
use project_core_domain::services::RegistrationService;

#[tokio::test]
async fn just_testing() {
    let conn = PgPool::connect("postgres://postgres:postgres@localhost:5432/mystarter")
        .await
        .expect("Failed to connect to the database.");
    let repo = UserRepositoryImpl::new(conn.clone());
    let service = RegistrationServiceImpl::new(repo);
    let user = service.try_register_user(RegisterUserModel {
        email: "foo@bar.com".to_string(),
        username: "foo".to_string(),
        password: "supersecrettestpassword1234#$$$!$@$@!".to_string(),
    }).await.expect("Failed to register user.");

    println!("{:#?}", user);
}