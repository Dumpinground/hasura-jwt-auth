use sqlx::PgPool;

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
    pub jwt_secret: String,
}
