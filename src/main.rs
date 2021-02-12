use untitled::establish_connection;
use untitled::operations::user::create;
use untitled::models::UserInfo;

fn main() {
    let connection = establish_connection();
    let user = UserInfo {
        email: "ezgoldner@gmail.com".to_string(),
        first_name: "Goldner".to_string(),
        last_name: "Ezra".to_string(),
        password: "password".to_string()
    };
    create(user, &connection).unwrap();
}
