-- Add up migration script here


CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL
);

INSERT INTO users(username, password) VALUES ('zbigniew', '123');
