use chrono::Local;
use core::types::user::*;
use core::types::RepoError;
use db::models::user::*;
use db::pool::*;
use db::schema::*;
use rocket_contrib::databases::diesel;
use rocket_contrib::databases::diesel::prelude::*;

pub struct PGUserRepo<'a> {
    db_conn: &'a DbConn,
}

impl<'a> PGUserRepo<'a> {
    pub fn new(conn: &'a DbConn) -> PGUserRepo<'a> {
        PGUserRepo { db_conn: conn }
    }
}

impl<'a> UserRepo for PGUserRepo<'a> {
    fn find_user_by_email(&self, email: &String) -> Result<Option<User>, RepoError> {
        let mut result_users = users::table
            .filter(users::email.eq(&email))
            .load::<QueryUser>(&*self.db_conn.master)?;

        if result_users.len() > 0 {
            let m = result_users.pop().unwrap();
            Ok(Some(User {
                username: m.username,
                email: m.email,
                password: m.password,
                token: m.token,
                bio: m.bio,
                image: m.image,
            }))
        } else {
            Ok(None)
        }
    }

    fn find_user_by_username(&self, username: &String) -> Result<Option<User>, RepoError> {
        let query = users::table.filter(users::username.eq(username));
        let mut result_users = query.load::<QueryUser>(&*self.db_conn.master)?;

        // let debug = diesel::debug_query::<diesel::mysql::Mysql, _>(&query);
        // let debug = format!("{:?}", debug);
        // println!("{:?}", debug);

        if result_users.len() > 0 {
            let m = result_users.pop().unwrap();
            Ok(Some(User {
                username: m.username,
                email: m.email,
                password: m.password,
                token: m.token,
                bio: m.bio,
                image: m.image,
            }))
        } else {
            Ok(None)
        }
    }

    fn save_new_user(&self, new_user: &NewUser) -> Result<usize, RepoError> {
        let date = Local::now();
        let date_time = date
            .format("%Y%m%d%H%M%S")
            .to_string()
            .parse::<u64>()
            .unwrap();

        let data_users = InsertUser {
            username: new_user.username.clone(),
            email: new_user.email.clone(),
            password: new_user.password.clone(),
            created_at: date_time,
            updated_at: date_time,
        };

        let result_users = diesel::insert_into(users::table)
            .values(&data_users)
            .execute(&*self.db_conn.master)?;

        Ok(result_users)
    }

    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>, RepoError> {
        let mut result_users = users::table
            .filter(users::email.eq(&credentials.email))
            .filter(users::password.eq(&credentials.password))
            .load::<QueryUser>(&*self.db_conn.master)?;

        println!("result_users {:?}", result_users);

        if result_users.len() > 0 {
            let m = result_users.pop().unwrap();
            Ok(Some(User {
                username: m.username,
                email: m.email,
                password: m.password,
                token: m.token,
                bio: m.bio,
                image: m.image,
            }))
        } else {
            Ok(None)
        }
    }
}
