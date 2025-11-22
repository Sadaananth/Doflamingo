#include "mainwindow.h"
#include "./ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    setWindowTitle("Doflamingo - Investment and Portfolio Manager");
    debt_widget = new DebtWidget(ui->debtTab);
    income_widget = new IncomeWidget(ui->incomeTab);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::OnOverviewMenu() {
}

void MainWindow::OnDebtMenu() {
}
