CREATE TABLE channels (
    channel_name VARCHAR(100) PRIMARY KEY,
    actively_logged boolean NOT NULL
);

CREATE TABLE users (
    user_login VARCHAR(100) PRIMARY KEY,
    opted_out boolean NOT NULL
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    channel VARCHAR(100) NOT NULL REFERENCES channels(channel_name),
    sender_login VARCHAR(100) NOT NULL REFERENCES users(user_login),
    post_timestamp INT NOT NULL,
    score INT NOT NULL
);