-- Your SQL goes here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    channel VARCHAR(100) NOT NULL,
    sender_login VARCHAR(100) NOT NULL,
    post_timestamp INT NOT NULL 
);
CREATE TABLE channels (
    id SERIAL PRIMARY KEY,
    channel_name VARCHAR(100) NOT NULL
)