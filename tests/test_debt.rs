use Doflamingo::database::database::DatabaseHandler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let database_handler = DatabaseHandler::new();
        database_handler.init_handler();

        database_handler.add_debt(
            "Test".to_string(),
            "start_date".to_string(),
            "end_date".to_string(),
            0u32,
            0.25,
            "tag".to_string(),
            "description".to_string(),
        );
        database_handler.add_debt(
            "Test1".to_string(),
            "start_date1".to_string(),
            "end_date1".to_string(),
            1u32,
            1.25,
            "tag1".to_string(),
            "description1".to_string(),
        );
        let debt_vec = database_handler.get_debt();
        assert_eq!(debt_vec.len(), 2);
        assert_eq!(debt_vec[0].start_date, "start_date");
        assert_eq!(debt_vec[0].end_date, "end_date");
        assert_eq!(debt_vec[0].amount, 0);
        // assert_eq!(debt_vec[0].interest, 0.25);
        assert_eq!(debt_vec[0].tag, "tag");
        assert_eq!(debt_vec[0].description, "description");

        assert_eq!(debt_vec[1].start_date, "start_date1");
        assert_eq!(debt_vec[1].end_date, "end_date1");
        assert_eq!(debt_vec[1].amount, 1);
        // assert_eq!(debt_vec[1].interest, 0.25);
        assert_eq!(debt_vec[1].tag, "tag1");
        assert_eq!(debt_vec[1].description, "description1");
    }
}
