use std::process::exit;

use sqlx::{postgres::PgPoolOptions, types::Uuid, PgPool};

use crate::types::word::Word;

pub async fn init_sql() -> sqlx::Pool<sqlx::Postgres> {
    // This project uses PostgreSQL
    match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://toakuai@localhost/toaq")
        .await
    {
        Err(x) => {
            println!("Error connecting to SQL Server!\n{}", x);
            exit(-1);
        }
        Ok(x) => x,
    }
}

pub async fn add_word(pool: &PgPool, word: Word<Uuid>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
INSERT INTO words
(id, author, word, definition, forked_from, lang, gloss, frame, created, edited)
VALUES
(uuid_generate_v4(), $1, $2, $3, $4, $5, $6, $7, $8, $9)
"#,
    )
    .bind(word.author)
    .bind(word.word)
    .bind(word.definition)
    .bind(word.forked_from)
    .bind(word.lang)
    .bind(word.gloss)
    .bind(word.frame)
    .bind(word.created)
    .bind(word.edited)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn remove_word(pool: &PgPool, word: Word<Uuid>) -> Result<(), sqlx::Error> {
    // AND clause is just here for safety
    sqlx::query("DELETE FROM words WHERE id = $1 AND word = $2")
        .bind(word.id)
        .bind(word.word)
        .execute(pool)
        .await?;

    Ok(())
}
