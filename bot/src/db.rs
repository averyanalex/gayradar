use sqlx::PgPool;

pub async fn update_user(db: &PgPool, id: i64) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
 INSERT INTO users ( id, last_activity )
     VALUES ( $1, CURRENT_TIMESTAMP )
     ON CONFLICT ( id ) DO UPDATE
         SET last_activity = CURRENT_TIMESTAMP
            "#,
        id,
    )
    .execute(db)
    .await?;
    Ok(())
}
