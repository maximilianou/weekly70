-- ./migrations/0001_bookstore_table.sql
CREATE TABLE book (
    title  varchar not null,
    author varchar not null,
    isbn   varchar not null
);

create unique index book_isbn_idx on book (isbn);
