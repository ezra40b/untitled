use crate::{
    schema::users::dsl::*,
    models::{User, UserInfo},
    errors::user::UserError,
    rand
};

use diesel::prelude::*;
use sodiumoxide::{
    crypto::pwhash::argon2id13
};
use diesel::insert_into;

pub fn create(new_user: UserInfo, connection: &PgConnection) -> Result<User, UserError> {
    let possible_user: Option<User> = users.filter(email.eq(new_user.email.clone())).first(connection).optional().unwrap();
    match possible_user {
        Some(..) => Err(UserError::new("Email is taken")),
        None => {
            sodiumoxide::init().unwrap();
            let random_id = rand(20);
            let hash = argon2id13::pwhash(
                new_user.password.as_bytes(),
                argon2id13::OPSLIMIT_INTERACTIVE,
                argon2id13::MEMLIMIT_INTERACTIVE
            ).unwrap();
            let packed = std::str::from_utf8(&hash.0).unwrap();
            println!("{}", packed);
            Ok(insert_into(users)
                .values((
                    random.eq(random_id),
                    first_name.eq(new_user.first_name),
                    last_name.eq(new_user.last_name),
                    email.eq(new_user.email),
                    password_hash.eq(
                        String::from(packed)
                    )
                ))
                .get_result::<User>(connection)
                .unwrap()
            )
        }
    }
}

pub fn auth(user: User, connection: &PgConnection) -> Result<User, UserError> {
    let possible_user: Option<User> = users.filter(
        random.eq(user.random)
            .and(email.eq(user.email)))
        .first(connection)
        .optional()
        .unwrap();

    match possible_user {
        None => Err(UserError::new("User not found.")),
        Some(result) => {
            sodiumoxide::init().unwrap();
            match argon2id13::HashedPassword::from_slice(result.password_hash.as_bytes()) {
                Some(hash) => {
                    if argon2id13::pwhash_verify(
                        &hash, user.password_hash.as_str().as_bytes()) {
                        Ok(result)
                    }
                    else {
                        Err(UserError::new("Passwords do not match."))
                    }
                },
                _ => {
                    Err(UserError::new("There was an error retrieving your password. Please try again."))
                }
            }
        }
    }
}