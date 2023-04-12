use diesel::{prelude::*, result::Error};

use crate::models::User;
use crate::schema::users::dsl::*;
pub struct UserRepository;

impl UserRepository {
    pub fn count_users(conn: &mut PgConnection) -> Result<i64, Error> {
        Ok(users.count().get_result(conn)?)
    }

    pub fn create_user(
        conn: &mut PgConnection,
        username_to_insert: &str,
        email_to_insert: &str,
    ) -> Result<User, Error> {
        let new_user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: String::from(username_to_insert),
            email: String::from(email_to_insert),
        };
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)
    }

    pub fn get_user(conn: &mut PgConnection, user_id: String) -> Result<Option<User>, Error> {
        users.find(user_id).first(conn).optional()
    }

    pub fn update_user(conn: &mut PgConnection, updated_user: &User) -> Result<User, Error> {
        diesel::update(users.find(&updated_user.id))
            .set((
                username.eq(&updated_user.username),
                email.eq(&updated_user.email),
            ))
            .get_result(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: String) -> Result<usize, Error> {
        diesel::delete(users.find(user_id)).execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use crate::create_connection_pool;
    use dotenv::dotenv;

    use super::*;

    #[test]
    fn create_user_test() {
        dotenv().ok();

        let mut connection = create_connection_pool().get().unwrap();
        let test_username = "user";
        let test_email = "test@example.com";

        let user = UserRepository::create_user(&mut connection, test_username, test_email).unwrap();

        assert_eq!(user.username, test_username);
        assert_eq!(user.email, test_email);

        // Clean up: delete the user
        UserRepository::delete_user(&mut connection, user.id).unwrap();
    }
}
