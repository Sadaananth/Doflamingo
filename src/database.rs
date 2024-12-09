pub mod database {
    use duckdb::{params, Connection};

    pub struct DatabaseHandler {
        conn: Connection,
    }

    impl DatabaseHandler {
        pub fn new() -> Self {
            DatabaseHandler {
                conn: Connection::open_in_memory().expect("Failed to open connection"),
            }
        }
        pub fn init_handler(&self) {
            self.conn
                .execute_batch(
                    r"CREATE SEQUENCE seq;
          CREATE TABLE salary (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  date            TEXT NOT NULL,
                  salary          INTEGER,
                  bonus           INTEGER
                  );
         ",
                )
                .expect("Failed to create table");
        }
        pub fn add_salary(&self, name: String, date: String, salary: u32, bonus: u32) {
            println!(
                "Saving users {} salary {} bonus {} from date {}",
                name, salary, bonus, date
            );

            self.conn.execute(
                "INSERT INTO salary (name, date, salary, bonus) VALUES (?, ?, ?, ?)",
                params![name, date, salary, bonus],
            );
        }

        pub fn print(&self) {
            #[derive(Debug)]
            struct Salary {
                id: i32,
                name: String,
                date: String,
                salary: i32,
                bonus: i32,
            }

            let mut stmt = self
                .conn
                .prepare("SELECT id, name, date, salary, bonus FROM salary")
                .expect("Prepare  Failed");
            let salary_iter = stmt
                .query_map([], |row| {
                    Ok(Salary {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        date: row.get(2)?,
                        salary: row.get(3)?,
                        bonus: row.get(4)?,
                    })
                })
                .expect("Query Failed");

            for salary in salary_iter {
                println!("Found salary {:?}", salary.unwrap());
            }
        }
    }
}
