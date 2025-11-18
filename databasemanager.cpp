#include "DatabaseManager.h"
#include <QSqlError>
#include <QDebug>

DatabaseManager::DatabaseManager(QObject *parent) : QObject(parent)
{
    db = QSqlDatabase::addDatabase("QSQLITE");
    db.setDatabaseName("portfolio.db");
    if (!db.open()) {
        qDebug() << "Error opening database:" << db.lastError().text();
    } else {
        initialize();
    }
}

void DatabaseManager::initialize()
{
    QSqlQuery query;
    query.exec("CREATE TABLE IF NOT EXISTS debts ("
               "id INTEGER PRIMARY KEY AUTOINCREMENT, "
               "name TEXT, "
               "principal REAL, "
               "interest REAL, "
               "monthlyPayment REAL)");
}

bool DatabaseManager::addDebt(const QString &name, double principal, double interest, double monthlyPayment)
{
    QSqlQuery query;
    query.prepare("INSERT INTO debts (name, principal, interest, monthlyPayment) VALUES (?, ?, ?, ?)");
    query.addBindValue(name);
    query.addBindValue(principal);
    query.addBindValue(interest);
    query.addBindValue(monthlyPayment);
    if(!query.exec()) {
        qDebug() << "Failed to insert debt:" << query.lastError().text();
        return false;
    }
    return true;
}

QSqlQuery DatabaseManager::getAllDebts()
{
    QSqlQuery query;
    query.exec("SELECT name, principal, interest, monthlyPayment FROM debts");
    return query;
}
