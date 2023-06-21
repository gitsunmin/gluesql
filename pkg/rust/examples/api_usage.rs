#[cfg(feature = "sled-storage")]
mod api_usage {
    use gluesql::prelude::{Glue, SledStorage};

    pub async fn run() {
        let storage = SledStorage::new("data/mutable-api").unwrap();
        let mut glue = Glue::new(storage);

        let sqls = [
            "CREATE TABLE IF NOT EXISTS Paper (id INTEGER, color TEXT);",
            "INSERT INTO Paper VALUES (100, 'RED');",
            "INSERT INTO Paper VALUES (200, 'BLUE');",
            "UPDATE Paper SET color = 'GREEN' WHERE id = 100;",
            "SELECT * FROM Paper;",
            "DROP TABLE Paper;",
        ];

        for sql in sqls {
            glue.execute(sql).await.unwrap();
            println!("sql: {}", sql);
        }
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    futures::executor::block_on(api_usage::run());
}
