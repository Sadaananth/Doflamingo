pub mod user {
    use std::collections::LinkedList;

    pub struct Debt {
        pub start_date: String,
        pub end_date: String,
        pub amount: u32,
        pub interest: f32,
        pub tag: String,
        pub description: String,
    }

    pub struct Salary {
        date: String,
        amount: u32,
        bonus: u32,
    }

    pub struct User {
        name: String,
        debt: LinkedList<Debt>,
        salary: LinkedList<Salary>,
    }
}
