create table users
(
    id         serial
        primary key,
    username   varchar   not null,
    role       varchar   not null,
    created_at timestamp not null
);

alter table users
    owner to kellian;

