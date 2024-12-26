// pub mod database;

pub mod salary {
    use crate::database::database::DatabaseHandler;
    use std::rc::Rc;

    pub struct SalaryHandler {
        pub database_handler: Rc<DatabaseHandler>,
    }

    impl SalaryHandler {
        pub fn new(database_handler: Rc<DatabaseHandler>) -> Self {
            Self { database_handler }
        }

        pub fn add_salary(&self, name: String, date: String, salary: u32, bonus: u32) {
            self.database_handler.add_salary(name, date, salary, bonus);
        }
    }
}
