// Use Diesel's prelude for common imports
use diesel::prelude::*;

// Use the models and schema generated in the previous step
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;

// Define the repository struct and implement the CRUD operations
pub struct UserRepository;

impl UserRepository {
    // Create
    pub fn create_user(
        conn: &mut PgConnection,
        username_to_insert: &str,
        email_to_insert: &str,
    ) -> Result<User, diesel::result::Error> {
        let new_user = NewUser {
            username: username_to_insert,
            email: email_to_insert,
        };
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)
    }

    // Read
    pub fn get_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Option<User>, diesel::result::Error> {
        users.find(user_id).first(conn).optional()
    }

    // Update
    pub fn update_user(
        conn: &mut PgConnection,
        updated_user: &User,
    ) -> Result<User, diesel::result::Error> {
        diesel::update(users.find(updated_user.id))
            .set((
                username.eq(&updated_user.username),
                email.eq(&updated_user.email),
            ))
            .get_result(conn)
    }

    // Delete
    pub fn delete_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(users.find(user_id)).execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use crate::create_connection_pool;

    use super::*;

    #[test]
    fn create_user_test() {
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
