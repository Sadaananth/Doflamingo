#include "incomewidget.h"
#include <QFormLayout>
#include <QHeaderView>
#include <QPushButton>
#include <QDialog>
#include <QLineEdit>
#include <QDateEdit>
#include <QDialogButtonBox>

IncomeWidget::IncomeWidget(QWidget *parent) : QObject(parent), parentTab(parent) {
    QVBoxLayout *mainLayout = new QVBoxLayout(parentTab);

    incomeTable = new QTableWidget(parentTab);
    incomeTable->setColumnCount(4);
    incomeTable->setHorizontalHeaderLabels({"Name", "Amount", "Start Date", "End Date"});
    incomeTable->horizontalHeader()->setSectionResizeMode(QHeaderView::Stretch);
    mainLayout->addWidget(incomeTable);

    mainLayout->addWidget(incomeTable);

    QPushButton *addIncomeBtn = new QPushButton("Add Income", parentTab);
    mainLayout->addWidget(addIncomeBtn);

    QObject::connect(addIncomeBtn, &QPushButton::clicked, this, &IncomeWidget::openAddIncomeDialog);
}

void IncomeWidget::openAddIncomeDialog()
{
    QDialog dialog(parentTab);
    dialog.setWindowTitle("Add New Income");

    QFormLayout form(&dialog);

    QLineEdit *nameEdit = new QLineEdit(&dialog);
    QLineEdit *amountEdit = new QLineEdit(&dialog);
    QDateEdit *startDateEdit = new QDateEdit(QDate::currentDate(), &dialog);
    QDateEdit *endDateEdit = new QDateEdit(QDate::currentDate(), &dialog);

    startDateEdit->setCalendarPopup(true);
    endDateEdit->setCalendarPopup(true);

    form.addRow("Name:", nameEdit);
    form.addRow("Amount:", amountEdit);
    form.addRow("Start Date:", startDateEdit);
    form.addRow("End Date:", endDateEdit);

    QDialogButtonBox *buttons = new QDialogButtonBox(
        QDialogButtonBox::Ok | QDialogButtonBox::Cancel,
        Qt::Horizontal, &dialog
        );
    form.addWidget(buttons);

    QObject::connect(buttons, &QDialogButtonBox::accepted, &dialog, &QDialog::accept);
    QObject::connect(buttons, &QDialogButtonBox::rejected, &dialog, &QDialog::reject);

    if (dialog.exec() == QDialog::Accepted)
    {
        int row = incomeTable->rowCount();
        incomeTable->insertRow(row);

        incomeTable->setItem(row, 0, new QTableWidgetItem(nameEdit->text()));
        incomeTable->setItem(row, 1, new QTableWidgetItem(amountEdit->text()));
        incomeTable->setItem(row, 2, new QTableWidgetItem(startDateEdit->date().toString("yyyy-MM-dd")));
        incomeTable->setItem(row, 3, new QTableWidgetItem(endDateEdit->date().toString("yyyy-MM-dd")));
    }
}
