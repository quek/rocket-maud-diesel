use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

use dotenv::dotenv;
use std::env;


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        // let pool = request.guard::<State<DbPool>>()?;
        let pool =
            match <State<DbPool> as FromRequest>::from_request(request) {
                Outcome::Success(pool) => pool,
                Outcome::Failure(e) => return Outcome::Failure(e),
                Outcome::Forward(_) => return Outcome::Forward(()),
            };
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
