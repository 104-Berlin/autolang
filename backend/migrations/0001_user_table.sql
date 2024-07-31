create table users (
    id varchar(255) primary key,
    username varchar(255) not null,
    email varchar(255) not null,
    passwordhash varchar(255) not null
);