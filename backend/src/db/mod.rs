pub mod model;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

diesel_migrations::embed_migrations!();

pub fn run_migrations(conn: &SqliteConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

pub fn establish_connection() -> DbPool {
    if cfg!(test) {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create DB pool.");

        run_migrations(&pool.get().unwrap());
        pool
    } else {
        // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_url = "mktour.db".to_string();
        let manager: ConnectionManager<SqliteConnection> =
            ConnectionManager::<SqliteConnection>::new(&database_url);

        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create DB pool.")
    }
}
