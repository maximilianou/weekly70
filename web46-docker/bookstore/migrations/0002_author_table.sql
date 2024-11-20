-- ./migrations/0001_bookstore_table.sql
CREATE TABLE author (
    name   VARCHAR NOT NULL,
    author_id INTEGER NOT NULL
);

CREATE UNIQUE INDEX author_id_idx on author (author_id);

ALTER TABLE book
ADD COLUMN author_id INTEGER,
ADD CONSTRAINT fk_author
FOREIGN KEY (author_id) REFERENCES author(author_id);
