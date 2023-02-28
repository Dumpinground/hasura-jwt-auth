use dotenvy::dotenv;
use std::{env};
use sqlx::{
    PgPool,
    postgres::PgPoolOptions
};

use hasura_jwt_auth::db::init::{
    connect_and_migrate, get_org_table_info
};

#[test]
fn test_env_var() {
    dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET").expect("Env variable JWT_SECRET is not set");
    let db_url = env::var("DATABASE_URL").expect("Env variable DATABASE_URL is not set");

    print!("jwt_secret: {}\ndb: {}\n", jwt_secret, db_url);
}

#[tokio::test]
async fn db_init() -> std::io::Result<()> {
    dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET").expect("Env variable JWT_SECRET is not set");
    let db_url = env::var("DATABASE_URL").expect("Env variable DATABASE_URL is not set");

    let database_connections = env::var("DATABASE_CONNECTIONS").ok();
    let post_register_url = env::var("POST_REGISTER_URL").ok();
    let post_reset_password_url = env::var("POST_RESET_PASSWORD_URL").ok();
    let post_set_password_url = env::var("POST_SET_PASSWORD_URL").ok();
    let org_table_column = env::var("JWT_ORG_CUSTOM_CLAIM").ok();

    let pg_pool = connect_and_migrate(&db_url, database_connections).await.unwrap();
    let table_conn = get_org_table_info(org_table_column);

    Ok(())
}

#[tokio::test]
async fn postdb_init() -> Result<(), sqlx::Error> {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").expect("Env variable DATABASE_URL is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
    .bind(150_i64)
    .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
