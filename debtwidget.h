#ifndef DEBTWIDGET_H
#define DEBTWIDGET_H

#include <QObject>
#include <QWidget>
#include <QLabel>
#include <QLineEdit>
#include <QDoubleSpinBox>
#include <QPushButton>
#include <QTableWidget>
#include "databasemanager.h"

class DebtWidget : public QObject
{
    Q_OBJECT
public:
    explicit DebtWidget(QWidget *parent = nullptr);

private slots:
    void toggleAddDebtForm();
    void submitDebt();

private:
    QWidget *parentTab;
    QLabel *totalDebtLabel;
    double totalDebt;

    QPushButton *toggleFormButton;
    QWidget *addDebtForm;
    QLineEdit *debtNameEdit;
    QDoubleSpinBox *principalSpin;
    QDoubleSpinBox *interestSpin;
    QDoubleSpinBox *monthlyPaymentSpin;

    QTableWidget *debtTable;
    DatabaseManager *dbManager;

    void loadDebtsFromDatabase();

    void updateTotalDebt();
    void addDebtToTable(const QString &name, double principal, double interest, double monthlyPayment);
};

#endif // DEBTWIDGET_H
