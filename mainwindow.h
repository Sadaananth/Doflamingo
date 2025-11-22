#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include "debtwidget.h"
#include "incomewidget.h"

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void OnOverviewMenu();
    void OnDebtMenu();

private:
    Ui::MainWindow *ui;
    DebtWidget* debt_widget;
    IncomeWidget* income_widget;
};
#endif // MAINWINDOW_H
