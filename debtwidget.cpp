#include "debtwidget.h"
#include <QVBoxLayout>
#include <QFormLayout>
#include <QHeaderView>

DebtWidget::DebtWidget(QWidget *parent)
    : QObject(parent), parentTab(parent), totalDebt(0)
{
    QVBoxLayout *mainLayout = new QVBoxLayout(parentTab);

    totalDebtLabel = new QLabel("Total Debt: $0.00", parentTab);
    QFont font;
    font.setPointSize(14);
    font.setBold(true);
    totalDebtLabel->setFont(font);
    mainLayout->addWidget(totalDebtLabel);

    toggleFormButton = new QPushButton("Add Debt", parentTab);
    mainLayout->addWidget(toggleFormButton);

    addDebtForm = new QWidget(parentTab);
    QFormLayout *formLayout = new QFormLayout(addDebtForm);

    debtNameEdit = new QLineEdit(parentTab);
    principalSpin = new QDoubleSpinBox(parentTab);
    principalSpin->setMaximum(1e9);
    interestSpin = new QDoubleSpinBox(parentTab);
    interestSpin->setSuffix("%");
    interestSpin->setMaximum(100);
    monthlyPaymentSpin = new QDoubleSpinBox(parentTab);
    monthlyPaymentSpin->setMaximum(1e7);

    formLayout->addRow("Debt Name:", debtNameEdit);
    formLayout->addRow("Principal ($):", principalSpin);
    formLayout->addRow("Interest Rate (%):", interestSpin);
    formLayout->addRow("Monthly Payment ($):", monthlyPaymentSpin);

    QPushButton *submitBtn = new QPushButton("Submit", addDebtForm);
    formLayout->addWidget(submitBtn);

    addDebtForm->setVisible(false);
    mainLayout->addWidget(addDebtForm);

    debtTable = new QTableWidget(parentTab);
    debtTable->setColumnCount(4);
    debtTable->setHorizontalHeaderLabels({"Name", "Principal", "Interest %", "Monthly Payment"});
    debtTable->horizontalHeader()->setSectionResizeMode(QHeaderView::Stretch);
    mainLayout->addWidget(debtTable);

    connect(toggleFormButton, &QPushButton::clicked, this, &DebtWidget::toggleAddDebtForm);
    connect(submitBtn, &QPushButton::clicked, this, &DebtWidget::submitDebt);

    loadDebtsFromDatabase();
}

void DebtWidget::toggleAddDebtForm()
{
    addDebtForm->setVisible(!addDebtForm->isVisible());
}

void DebtWidget::submitDebt()
{
    QString name = debtNameEdit->text();
    double principal = principalSpin->value();
    double interest = interestSpin->value();
    double monthlyPayment = monthlyPaymentSpin->value();

    if (name.isEmpty() || principal <= 0)
        return;

    if (dbManager->addDebt(name, principal, interest, monthlyPayment)) {
        totalDebt += principal;
        updateTotalDebt();
        addDebtToTable(name, principal, interest, monthlyPayment);
    }

    debtNameEdit->clear();
    principalSpin->setValue(0);
    interestSpin->setValue(0);
    monthlyPaymentSpin->setValue(0);
    addDebtForm->setVisible(false);
}

void DebtWidget::loadDebtsFromDatabase()
{
    QSqlQuery query = dbManager->getAllDebts();
    while(query.next()) {
        QString name = query.value(0).toString();
        double principal = query.value(1).toDouble();
        double interest = query.value(2).toDouble();
        double monthlyPayment = query.value(3).toDouble();

        totalDebt += principal;
        addDebtToTable(name, principal, interest, monthlyPayment);
    }
    updateTotalDebt();
}

void DebtWidget::updateTotalDebt()
{
    totalDebtLabel->setText(QString("Total Debt: $%1").arg(totalDebt, 0, 'f', 2));
}

void DebtWidget::addDebtToTable(const QString &name, double principal, double interest, double monthlyPayment)
{
    int row = debtTable->rowCount();
    debtTable->insertRow(row);

    debtTable->setItem(row, 0, new QTableWidgetItem(name));
    debtTable->setItem(row, 1, new QTableWidgetItem(QString::number(principal, 'f', 2)));
    debtTable->setItem(row, 2, new QTableWidgetItem(QString::number(interest, 'f', 2)));
    debtTable->setItem(row, 3, new QTableWidgetItem(QString::number(monthlyPayment, 'f', 2)));
}
