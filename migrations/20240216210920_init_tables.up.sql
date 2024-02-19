create table users (
    id bigserial primary key,
    username varchar(255) not null unique,
    protected_password text not null,
    email varchar(255) not null unique,
    created_at timestamptz not null default now()
);
