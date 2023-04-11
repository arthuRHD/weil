#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection;
    use diesel::result::Error;

    #[test]
    fn create_user_test() {
        let connection = establish_connection();
        let username = "test_user";
        let email = "test@example.com";

        let user = UserRepository::create_user(&connection, username, email).unwrap();

        assert_eq!(user.username, username);
        assert_eq!(user.email, email);

        // Clean up: delete the user
        UserRepository::delete_user(&connection, user.id).unwrap();
    }
}
