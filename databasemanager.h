#ifndef DATABASEMANAGER_H
#define DATABASEMANAGER_H

#include <QObject>
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QVariant>

class DatabaseManager : public QObject
{
    Q_OBJECT
public:
    explicit DatabaseManager(QObject *parent = nullptr);
    bool addDebt(const QString &name, double principal, double interest, double monthlyPayment);
    QSqlQuery getAllDebts();

private:
    QSqlDatabase db;
    void initialize();
};

#endif // DATABASEMANAGER_H
