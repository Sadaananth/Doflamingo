pub mod salary {
    use crate::database::database::DatabaseHandler;

    pub struct SalaryHandler {
        database_handler: DatabaseHandler,
    }

    impl SalaryHandler {
        pub fn new(database_handler: DatabaseHandler) -> Self {
            SalaryHandler { database_handler }
        }

        pub fn add_salary(&self, date: String, amount: u32) {
            self.database_handler
                .add_salary("Test".to_string(), date, amount, 0u32);
            self.database_handler.print();
        }
    }
}
