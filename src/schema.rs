table! {
    users (id) {
        id -> Int4,
        random -> Bpchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password_hash -> Text,
    }
}
