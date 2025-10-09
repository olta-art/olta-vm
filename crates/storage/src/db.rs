use sqlx::{PgPool, Row};
use anyhow::{Result, Context};
use serde_json;

/// database connection and operations for lobby persistence
#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

#[derive(Debug, Clone)]
pub enum DbOperation {
    SaveProcessState { process_id: String, full_state: String, is_hot: bool },
}

impl Database {
    /// create new database connection with pool
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url)
            .await
            .context("Failed to connect to PostgreSQL")?;
            
        Ok(Self { pool })
    }

    /// run database migrations
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!()
            .run(&self.pool)
            .await
            .context("Failed to run migrations")?;
        Ok(())
    }

    /// get the underlying pool for advanced operations
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Save complete lobby state to database
    pub async fn save_process_state(
        &self,
        process_id: &str,
        full_state: &str,
        is_hot: bool,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO processes (process_id, full_state, is_hot, last_activity)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (process_id) 
            DO UPDATE SET 
                full_state = $2,
                is_hot = $3,
                last_activity = NOW()
            "#
        )
        .bind(process_id)
        .bind(serde_json::from_str::<serde_json::Value>(full_state)
            .context("Invalid JSON in full_state")?)
        .bind(is_hot)
        .execute(&self.pool)
        .await
        .context("Failed to save process state")?;

        Ok(())
    }

    /// load process state from database
    pub async fn load_process_state(&self, process_id: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT full_state FROM processes WHERE process_id = $1")
            .bind(process_id)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to load process state")?;

        Ok(row.map(|r| r.get::<serde_json::Value, _>("full_state").to_string()))
    }

}


/// background worker for database operations
pub struct DatabaseWorker {
    database: Database,
    receiver: tokio::sync::mpsc::UnboundedReceiver<DbOperation>,
}

impl DatabaseWorker {
    pub fn new(database: Database, receiver: tokio::sync::mpsc::UnboundedReceiver<DbOperation>) -> Self {
        Self { database, receiver }
    }

    /// Run the background worker
    pub async fn run(mut self) {
        println!("db worker started");
        
        while let Some(operation) = self.receiver.recv().await {
            if let Err(e) = self.process_operation(operation).await {
                eprintln!("db operation failed: {}", e);
                // TODO: implement retry logic with exponential backoff
            }
        }
        
        println!("db worker stopped");
    }

    async fn process_operation(&self, operation: DbOperation) -> Result<()> {
        match operation {
            DbOperation::SaveProcessState { process_id, full_state, is_hot } => {
                self.database.save_process_state(&process_id, &full_state, is_hot).await?;
            }
        }
        Ok(())
    }

}