pub mod psql_connection {
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Pool;
    use sqlx::Postgres;
    use crate::data_structures::Thread;
    use crate::data_structures::ThreadReply;
  
    pub async fn gen_pol(url: &str) -> Pool<Postgres> {
      let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .unwrap();
      pool
    }

    pub async fn add_thread(pool: &Pool<Postgres>, thread: Thread) -> Result<(), sqlx::Error> {

      let _: (i64, ) = sqlx::query_as("INSERT INTO threads (title, thread_url, image_url) VALUES ($1, $2, $3)")
                                .bind(thread.title)
                                .bind(thread.url)
                                .bind(thread.image_href)
                                .fetch_one(pool)
                                .await?;
      Ok(())
    }

    pub async fn add_reply(pool: Pool<Postgres>, reply: ThreadReply, thread_id: i64) -> Result<(), sqlx::Error> {
      let _: (i64, ) = sqlx::query_as("INSERT INTO replys (thread, content, image_url, is_op) VALUES ($1, $2, $3)")
                                .bind(thread_id)
                                .bind(reply.subject)
                                .bind(reply.image_response)
                                .bind(reply.is_op)
                                .fetch_one(&pool)
                                .await?;
      Ok(())
    }

  }