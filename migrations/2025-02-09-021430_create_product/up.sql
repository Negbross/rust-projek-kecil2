-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    product_name varchar(500) NOT NULL unique ,
    description varchar(5000) not null ,
    quantity integer not null
)