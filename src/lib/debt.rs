pub mod debt {
    use crate::database::database::DatabaseHandler;

    pub struct DebtHandler {
        database_handler: DatabaseHandler,
    }

    impl DebtHandler {
        pub fn new(database_handler: DatabaseHandler) -> Self {
            DebtHandler { database_handler }
        }

        pub fn add_debt(&self, start_date: String, end_date: String, amount: u32, interest: f32, tag: String, description: String) {
            self.database_handler
                .add_debt("Test".to_string(), start_date, end_date, amount, interest, tag, description);
            self.database_handler.print();
        }
    }
}
