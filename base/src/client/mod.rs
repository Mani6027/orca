
use futures::executor;

pub mod database;
pub mod redis;

#[derive(Debug, Clone, Default)]
pub struct Client {
    database: Option<database::Database>,
    redis: Option<redis::Redis>,
}

impl Client {
    pub fn default() -> Self {
        Client {
            database: None,
            redis: None,
        }
    }

    pub fn database(mut self) -> database::Database {
        if let Some(x) = self.database.clone() {
            return x;
        } else {
            let database = executor::block_on(database::Database::new(super::CONFIG.database.url.clone()));
            self.database = Some(database.clone());
            return database;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let cli = super::Client::default();
        let database = cli.database();
    }
}
