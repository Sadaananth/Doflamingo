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
    let database_handler = DatabaseHandler::new();
    database_handler.init_handler();
    
    let salary_handler = SalaryHandler::new(database_handler);
    
    let today = Local::now();
    let ui = AppWindow::new()?;
    ui.global::<Data>().set_this_year(today.year());
    ui.global::<Data>().set_this_month(today.month().try_into().unwrap());
    ui.global::<Data>().set_today(today.day().try_into().unwrap());
    ui.on_add_salary(move |date, salary| {
        salary_handler.add_salary(date.to_string(), salary.try_into().unwrap());
    });
    ui.run()?;

    Ok(())
}
