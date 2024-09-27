\c main;

INSERT INTO roles (name)
VALUES ('admin'),
       ('moderator'),
       ('user');


INSERT INTO users (username, role_id, created_at) VALUES
('alice', 1, '2024-01-01 10:00:00'),
('bob', 3, '2024-01-02 11:00:00'),
('charlie', 3, '2024-01-03 12:00:00'),
('dave', 2, '2024-01-04 13:00:00'),
('eve', 2, '2024-01-05 14:00:00'),
('frank', 1, '2024-01-06 15:00:00'),
('grace', 2, '2024-01-07 16:00:00'),
('heidi', 3, '2024-01-08 17:00:00'),
('ivan', 1, '2024-01-09 18:00:00'),
('judy', 3, '2024-01-10 19:00:00');
