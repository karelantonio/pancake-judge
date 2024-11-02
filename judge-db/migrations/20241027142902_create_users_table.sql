
-- The users db must contain:
-- ID (Primary key)
-- Username
-- Name
-- Email
-- Password Hash
-- Password Salt
-- Join date
-- Problems (Solved problems)
CREATE TABLE users(
    id  BIGINT PRIMARY KEY NOT NULL,
    username    VARCHAR NOT NULL,
    "name"  VARCHAR NOT NULL,
    email   VARCHAR DEFAULT NULL,
    password_hash   VARCHAR DEFAULT NULL,
    password_salt   VARCHAR DEFAULT NULL,
    join_date   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    problems    INTEGER NOT NULL DEFAULT 0
);

-- Now we must insert Root user
INSERT INTO users(id, username, "name") VALUES (0, 'root', 'System administrator');