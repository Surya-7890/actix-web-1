CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    year INT NOT NULL,
    author_id INT NOT NULL REFERENCES authors(id),
    price INT NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    price INT NOT NULL
);

CREATE TABLE book_orders (
    order_id INT NOT NULL REFERENCES orders(id) ON DELETE CASCADE, 
    book_id INT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    quantity INT NOT NULL,
    PRIMARY KEY (order_id, book_id)
);