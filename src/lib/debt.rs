// pub mod database;

pub mod debt {
    use crate::database::database::DatabaseHandler;
    use std::rc::Rc;

    pub struct DebtHandler {
        pub database_handler: Rc<DatabaseHandler>,
    }

    impl DebtHandler {
        pub fn new(database_handler: Rc<DatabaseHandler>) -> Self {
            Self { database_handler }
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
            self.database_handler.add_debt(
                name,
                start_date,
                end_date,
                amount,
                interest,
                tag,
                description,
            );
        }
    }
}
