-- Your SQL goes here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    username TEXT,
    email TEXT,
    img_url TEXT,
    phone NUMBER,
    password TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
); 