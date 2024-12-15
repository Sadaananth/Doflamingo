pub mod database {
    use crate::user::user::Debt;
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
                    r"
          CREATE TABLE salary (
                  name            TEXT NOT NULL PRIMARY KEY,
                  date            TEXT NOT NULL,
                  salary          INTEGER,
                  bonus           INTEGER
                  );
         ",
                )
                .expect("Failed to create table salary");
            self.conn
                .execute_batch(
                    r"
            CREATE TABLE debt (
                    name            TEXT NOT NULL PRIMARY KEY,
                    start_date      TEXT NOT NULL,
                    end_date        TEXT NOT NULL,
                    amount          INTEGER,
                    interest        INTEGER,
                    tag             TEXT NOT NULL,
                    description     TEXT NOT NULL
                    );
            ",
                )
                .expect("Failed to create table debt");
        }

        pub fn add_salary(&self, name: String, date: String, salary: u32, bonus: u32) {
            println!(
                "Saving users {} salary {} bonus {} from date {}",
                name, salary, bonus, date
            );

            let _ = self.conn.execute(
                "INSERT INTO salary (name, date, salary, bonus) VALUES (?, ?, ?, ?)",
                params![name, date, salary, bonus],
            );
        }

        pub fn add_debt(
            &self,
            name: String,
            start_date: String,
            end_date: String,
            amount: u32,
            interest: f32,
            tag: String,
            description: String,
        ) {
            println!(
                "Saving users {} start date {} end date {} amount {} interest {} tag {} description {}",
                name, start_date, end_date, amount, interest, tag, description
            );

            let _ = self.conn.execute(
                "INSERT INTO debt (name, start_date, end_date, amount, interest, tag, description) VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![name, start_date, end_date, amount, interest, tag, description],
            );
        }

        pub fn get_debt(&self) -> Vec<Debt> {

            let mut stmt = self
                .conn
                .prepare("SELECT name, start_date, end_date, amount, interest, tag, description FROM debt")
                .expect("Prepare  Failed");
            let debt_iter = stmt
                .query_map([], |row| {
                    Ok(Debt {
                        start_date: row.get(1)?,
                        end_date: row.get(2)?,
                        amount: row.get(3)?,
                        interest: row.get(4)?,
                        tag: row.get(5)?,
                        description: row.get(6)?,
                    })
                })
                .expect("Query Failed");

            let mut data = Vec::new();
            for row in debt_iter {
                data.push(row.unwrap());
            }
            data
        }

        pub fn print(&self) {
            #[derive(Debug)]
            struct Salary {
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
                        name: row.get(0)?,
                        date: row.get(1)?,
                        salary: row.get(2)?,
                        bonus: row.get(3)?,
                    })
                })
                .expect("Query Failed");

            for salary in salary_iter {
                println!("Found salary {:?}", salary.unwrap());
            }
        }
    }
}
