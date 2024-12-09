#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use duckdb::Result;
use std::error::Error;
use chrono::prelude::*;

slint::include_modules!();

mod salary;
use salary::salary::SalaryHandler;

mod database;
use database::database::DatabaseHandler;

fn main() -> Result<(), Box<dyn Error>> {
    let today = Local::now();
    let database_handler = DatabaseHandler::new();
    database_handler.init_handler();

    let salary_handler = SalaryHandler::new(database_handler);

    let ui = AppWindow::new()?;
    ui.on_add_salary(move |date, salary| {
        salary_handler.add_salary(date.to_string(), salary.try_into().unwrap());
    });
    ui.run()?;

    Ok(())
}
