use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::users::UpdateUser;
use crate::models::users::{User, NewUser};
use crate::schema::users::dsl::*;

pub enum UserErrors {
    IncorrectPassword,
    DatabaseError(diesel::result::Error)
}

impl UserErrors {
    pub fn to_string(&self) -> String {
        match self {
            UserErrors::IncorrectPassword => "incorrect password".to_string(),
            UserErrors::DatabaseError(err) => err.to_string()
        }
    }
}

impl User {
    pub fn user_signup(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users)
            .values(new_user)
            .get_result(conn)
    }

    pub fn user_login(conn: &mut PgConnection, user_name: &str, user_password: &str) -> Result<User, UserErrors> {
        let user = users.filter(username.eq(user_name.to_string()))
            .first::<User>(conn);

        match user {
            Ok(user) => {
                if user.password == user_password {
                    Ok(user)
                } else {
                    Err(UserErrors::IncorrectPassword)
                }
            }
            Err(err) => {
                Err(UserErrors::DatabaseError(err))
            }
        }
    }

    pub fn update_user(conn: &mut PgConnection, user_id: i32, updated_user: &UpdateUser) -> Result<User, diesel::result::Error> {
        diesel::update(users.filter(id.eq(user_id)))
            .set(updated_user)
            .get_result(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<User, diesel::result::Error> {
        diesel::delete(users.filter(id.eq(user_id))).get_result(conn)
    }
}