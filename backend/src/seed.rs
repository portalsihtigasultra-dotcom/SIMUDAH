use std::path::Path;

use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::Argon2;

struct SeedUser {
    username: &'static str,
    password: &'static str,
    role: &'static str,
}

#[tokio::main]
async fn main() {
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    println!("Loading .env from {:?}", env_path);
    dotenvy::from_path(&env_path).ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    let users = vec![
        SeedUser { username: "petugas1", password: "123456", role: "petugas" },
        SeedUser { username: "petugas2", password: "123456", role: "petugas" },
        SeedUser { username: "verifikator1", password: "123456", role: "verifikator" },
        SeedUser { username: "verifikator2", password: "123456", role: "verifikator" },
        SeedUser { username: "validator1", password: "123456", role: "validator" },
    ];

    for u in &users {
        let existing: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM users WHERE username = $1")
                .bind(u.username)
                .fetch_optional(&pool)
                .await
                .expect("Query failed");

        if existing.is_some() {
            println!("SKIP  {} — already exists", u.username);
            continue;
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(u.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        sqlx::query(
            "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3::user_role)",
        )
        .bind(u.username)
        .bind(&password_hash)
        .bind(u.role)
        .execute(&pool)
        .await
        .expect("Insert failed");

        println!("OK    {} ({})", u.username, u.role);
    }

    println!("\nSeeding complete!");
}
