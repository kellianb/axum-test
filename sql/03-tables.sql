\c main;

CREATE TABLE "messages"
(
    "id"        serial PRIMARY KEY,
    "sent_at"   timestamp    not null,
    "content"   varchar(255) not null,
    "sender_id" int          not null,
    "deleted"   bool         not null default FALSE
);

CREATE TABLE "users"
(
    "id"            serial PRIMARY KEY,
    "username"      varchar(255) not null UNIQUE,
    "password_hash" varchar(128) not null,
    "role_id"       int          not null,
    "created_at"    timestamp    not null,
    "deleted"       bool         not null default FALSE
);

CREATE TABLE "roles"
(
    "id"   serial PRIMARY KEY,
    "name" varchar(255) not null
);

CREATE TABLE "sessions"
(
    "token"      varchar(48) not null,
    "user_id"    int         not null,
    "created_at" timestamp   not null,
    "expires_at" timestamp   not null
);

ALTER TABLE "users"
    ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");

ALTER TABLE "messages"
    ADD FOREIGN KEY ("sender_id") REFERENCES "users" ("id");

ALTER TABLE "sessions"
    ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

alter table messages
    owner to kellian;

alter table users
    owner to kellian;

alter table roles
    owner to kellian;

alter table sessions
    owner to kellian;
