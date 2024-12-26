#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use duckdb::Result;
use std::error::Error;
use std::rc::Rc;
use Doflamingo::database;

slint::include_modules!();

use Doflamingo::database::database::DatabaseHandler;
use Doflamingo::debt::debt::DebtHandler;
use Doflamingo::salary::salary::SalaryHandler;

fn main() -> Result<(), Box<dyn Error>> {
    let database_handler = Rc::new(DatabaseHandler::new(true));
    database_handler.init_handler();

    let salary_handler = SalaryHandler::new(Rc::clone(&database_handler));
    let debt_handler = DebtHandler::new(Rc::clone(&database_handler));

    let today = Local::now();
    let ui = AppWindow::new()?;
    ui.global::<Data>().set_this_year(today.year());
    ui.global::<Data>()
        .set_this_month(today.month().try_into().unwrap());
    ui.global::<Data>()
        .set_today(today.day().try_into().unwrap());
    ui.on_add_salary({
        move |date, salary| {
            salary_handler.add_salary(
                "Test".to_string(),
                date.to_string(),
                salary.try_into().unwrap(),
                0u32,
            );
        }
    });
    ui.on_add_debt({
        move |start_date, end_date, amount, interest, description| {
            debt_handler.add_debt(
                "Test".to_string(),
                start_date.to_string(),
                end_date.to_string(),
                amount.to_string().parse().unwrap(),
                interest.to_string().parse().unwrap(),
                "tag".to_string(),
                description.to_string(),
            );
            debt_handler.database_handler.print_debt();
        }
    });
    ui.run()?;

    Ok(())
}
