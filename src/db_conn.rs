use diesel::r2d2::{ self, ConnectionManager };
use diesel::PgConnection;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DBPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be a string");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}