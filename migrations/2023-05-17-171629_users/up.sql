-- Your SQL goes here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    username TEXT NOT NULL,
    email TEXT,
    img_url TEXT,
    phone NUMBER,
    password TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
); 