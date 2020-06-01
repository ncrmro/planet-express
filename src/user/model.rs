use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use fake::{faker::internet, Fake};

use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

use argon2::{self, Config};
use paperclip::actix::Apiv2Schema;
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct UserAuth {
    pub id: Option<i32>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// Implementation for User struct, functions for read/write/update and delete User from database
impl User {
    pub async fn authenticate(obj: UserAuth, conn: &PgPool) -> Result<AuthResponse> {
        let email = obj.email;
        let rec = sqlx::query!(
            "SELECT id, email, password FROM users WHERE email = $1",
            email
        )
        .fetch_one(conn)
        .await?;
        let matches = argon2::verify_encoded(&rec.password, obj.password.as_ref()).unwrap();
        assert!(matches);
        let user = User {
            id: rec.id,
            email: rec.email,
        };
        Ok(AuthResponse {
            token: super::auth::jwt_get(user.id),
            user,
        })
    }
    pub async fn create(obj: &UserAuth, conn: &PgPool) -> Result<AuthResponse> {
        let mut tx = conn.begin().await?;
        let salt = b"randomsalt";
        let hash = argon2::hash_encoded(obj.password.as_ref(), salt, &Config::default()).unwrap();
        let rec =
            sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email")
                .bind(&obj.email)
                .bind(hash)
                .map(|row: PgRow| User {
                    id: row.get(0),
                    email: row.get(1),
                })
                .fetch_one(&mut tx)
                .await?;

        tx.commit().await?;
        let user = User {
            id: rec.id,
            email: rec.email,
        };
        Ok(AuthResponse {
            token: super::auth::jwt_get(user.id),
            user,
        })
    }
}

pub struct UserFactory {}

impl UserFactory {
    pub fn build() -> UserAuth {
        UserAuth {
            id: None,
            email: internet::en::FreeEmail().fake(),
            password: "testpassword".to_string(),
        }
    }

    pub async fn new(pool: PgPool) -> UserAuth {
        let obj_build = UserFactory::build();
        let obj = User::create(&obj_build, pool.borrow()).await.unwrap();
        UserAuth {
            id: None,
            email: obj.user.email,
            password: obj_build.password,
        }
    }
}
