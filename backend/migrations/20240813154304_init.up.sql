-- Add up migration script here


CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL
);

INSERT INTO users(username, password) VALUES ('zbigniew', '123');

CREATE TABLE notes(
    id SERIAL PRIMARY KEY,
    creator_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,   
    time_created BIGINT NOT NULL,
    time_edited BIGINT NOT NULL
);
