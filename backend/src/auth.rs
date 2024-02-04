use crate::model::User;
use actix_session::Session;
use actix_web::web::{Data, Form};
use actix_web::{error::ErrorUnauthorized, HttpResponse};
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::{MySql, Pool};

#[derive(serde::Serialize)]
pub struct SessionDetails {
    user_id: u32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuthorizedUser {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
    pub approved: bool,
}

pub fn check_auth(session: &Session) -> Result<u32, actix_web::Error> {
    match session.get::<u32>("user_id").unwrap() {
        Some(user_id) => Ok(user_id),
        None => Err(ErrorUnauthorized("User not logged in.")),
    }
}

pub async fn register_user(
    data: Form<User>,
    pool: Data<Pool<MySql>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let data = data.into_inner();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(data.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // Use to verify.
    // let parsed_hash = PasswordHash::new(&hash).unwrap();

    const INSERT_QUERY: &str =
        "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING id;";

    let fetch_one: Result<(u32,), sqlx::Error> = sqlx::query_as(INSERT_QUERY)
        .bind(data.username)
        .bind(password_hash)
        .fetch_one(&mut pool.acquire().await.unwrap())
        .await;

    match fetch_one {
        Ok((user_id,)) => Ok(user_id.to_string()),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn login_user(
    session: Session,
    data: Form<User>,
    pool: Data<Pool<MySql>>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let data = data.into_inner();
    let fetched_user: AuthorizedUser = match sqlx::query_as(
        "SELECT id, username, password_hash, approved FROM users WHERE username = ?;",
    )
    .bind(data.username)
    .fetch_one(&mut pool.acquire().await?)
    .await
    {
        Ok(fetched_user) => fetched_user,
        Err(e) => return Ok(HttpResponse::NotFound().body(format!("{e:?}"))),
    };

    let parsed_hash = PasswordHash::new(&fetched_user.password_hash).unwrap();

    match Argon2::default().verify_password(&data.password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            session.insert("user_id", &fetched_user.id)?;
            session.renew();

            Ok(HttpResponse::Ok().json(SessionDetails {
                user_id: fetched_user.id,
            }))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().body("Incorrect password.")),
    }
}

pub async fn logout_user(session: Session) -> HttpResponse {
    if check_auth(&session).is_err() {
        return HttpResponse::NotFound().body("No user logged in.");
    }

    session.purge();
    HttpResponse::SeeOther()
        .append_header(("Location", "/login"))
        .body(format!("User logged out successfully."))
}
