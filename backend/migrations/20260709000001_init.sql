CREATE TYPE data_status AS ENUM ('mentah', 'terverifikasi', 'tervalidasi', 'terkoreksi', 'ditolak');

CREATE TYPE user_role AS ENUM ('petugas', 'verifikator', 'validator');

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'petugas',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
