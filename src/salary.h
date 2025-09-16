#ifndef SAVINGSCALCULATOR_H
#define SAVINGSCALCULATOR_H

#include <QObject>

class SavingsCalculator : public QObject {
    Q_OBJECT
    Q_PROPERTY(double salary READ salary WRITE setSalary NOTIFY salaryChanged)
    Q_PROPERTY(double percentage READ percentage WRITE setPercentage NOTIFY percentageChanged)
    Q_PROPERTY(double savings READ savings NOTIFY savingsChanged)

public:
    explicit SavingsCalculator(QObject *parent = nullptr);

    double salary() const;
    void setSalary(double value);

    double percentage() const;
    void setPercentage(double value);

    double savings() const;

signals:
    void salaryChanged();
    void percentageChanged();
    void savingsChanged();

private:
    double m_salary = 0;
    double m_percentage = 0;

    void updateSavings();
    double m_savings = 0;
};

#endif // SAVINGSCALCULATOR_H
