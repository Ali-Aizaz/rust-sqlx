create table book (
  isbn varchar not null primary key,
  title varchar not null,
  author varchar not null
);

create table author (
  id BIGSERIAL PRIMARY KEY,
  name varchar not null
)