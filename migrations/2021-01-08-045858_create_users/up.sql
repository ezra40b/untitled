CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    random char(20) NOT NULL,
    first_name varchar(30) NOT NULL,
    last_name varchar(30) NOT NULL,
    email varchar(300) NOT NULL,
    password_hash TEXT NOT NULL
)