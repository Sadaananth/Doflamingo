#ifndef INCOMEWIDGET_H
#define INCOMEWIDGET_H

#include <QObject>
#include <QTableWidget>

class IncomeWidget : public QObject
{
    Q_OBJECT
public:
    explicit IncomeWidget(QWidget *parent = nullptr);

private slots:
    void openAddIncomeDialog();

private:
    QWidget *parentTab;
    QTableWidget *incomeTable;
};

#endif // INCOMEWIDGET_H
