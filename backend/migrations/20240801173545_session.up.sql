-- Add up migration script here
create table session (
    user_id varchar(255) not null references users(id) on delete cascade,
    token varchar(255) not null,
    expiry timestamp not null
);