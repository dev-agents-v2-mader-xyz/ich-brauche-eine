use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect(database_url: &str, schema: &str) -> Result<PgPool, sqlx::Error> {
    let schema = schema.to_string();
    let options = PgPoolOptions::new()
        .max_connections(10)
        .after_connect(move |conn, _meta| {
            let schema = schema.clone();
            Box::pin(async move {
                sqlx::query(&format!("SET search_path TO {schema}"))
                    .execute(conn)
                    .await?;
                Ok(())
            })
        });
    options.connect(database_url).await
}
