\c main;

INSERT INTO roles (name)
VALUES ('admin'),
       ('moderator'),
       ('user');

-- The password for all test users is 123321
INSERT INTO users (username, password_hash, role_id, created_at) VALUES
('John', '$argon2id$v=19$m=19456,t=2,p=1$HdaU+syjZcOlv8tzNJ4LMQ$xzLhx6dObk4p1565R9A+lO9cAi9pT6WKDbDFDyONNH8', 1, '2024-10-15 07:36:55.453511'),
('Joe', '$argon2id$v=19$m=19456,t=2,p=1$J8tKDQlTRSxcQKsSzPD1LQ$nRgnXxU9ckY5We3XCR/auJVSZ4RWduqol4b3gFuTd4c', 1, '2024-10-15 07:37:15.661956'),
('Luke', '$argon2id$v=19$m=19456,t=2,p=1$ZjFLuhsuk3iY+pjw43y2PA$euXotJAzL+3oIgUhKM2PT5w64pL23sfD4pQ+o7PMdi0', 1, '2024-10-15 07:37:48.945183'),
('Alice', '$argon2id$v=19$m=19456,t=2,p=1$sm7ecGQHufz0HMCZQ/UibA$2j8/FoApU1ChLRXJgdffwrGVCREEjAKJ8phV2sgROsM', 1, '2024-10-15 07:37:52.197724');
