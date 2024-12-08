use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;

pub(crate) struct V1Migration;

impl Migration<sqlx::Any> for V1Migration {
    fn app(&self) -> &str {
        "atbb"
    }
    fn name(&self) -> &str {
        "V1Migration"
    }
    fn parents(&self) -> Vec<Box<dyn Migration<sqlx::Any>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<sqlx::Any>>> {
        vec![Box::new(CreateSchemaOperation)]
    }
}

struct CreateSchemaOperation;

#[async_trait::async_trait]
impl Operation<sqlx::Any> for CreateSchemaOperation {
    // Up function runs apply migration
    async fn up(&self, connection: &mut sqlx::AnyConnection) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE forums (
                id INTEGER PRIMARY KEY NOT NULL,
                rkey TEXT NOT NULL,
                cid TEXT NOT NULL,
                name TEXT(300) NOT NULL,
                description TEXT(300)
            );
            CREATE TABLE posts (
                id INTEGER PRIMARY KEY NOT NULL,
                rkey TEXT NOT NULL,
                cid TEXT NOT NULL
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut sqlx::AnyConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE posts; DROP TABLE forums;")
            .execute(connection)
            .await?;
        Ok(())
    }
}
