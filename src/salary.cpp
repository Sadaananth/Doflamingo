#include "salary.h"

SavingsCalculator::SavingsCalculator(QObject *parent)
    : QObject(parent) {}

double SavingsCalculator::salary() const { return m_salary; }

void SavingsCalculator::setSalary(double value) {
    if (m_salary != value) {
        m_salary = value;
        emit salaryChanged();
        updateSavings();
    }
}

double SavingsCalculator::percentage() const { return m_percentage; }

void SavingsCalculator::setPercentage(double value) {
    if (m_percentage != value) {
        m_percentage = value;
        emit percentageChanged();
        updateSavings();
    }
}

double SavingsCalculator::savings() const { return m_savings; }

void SavingsCalculator::updateSavings() {
    double newSavings = (m_salary * m_percentage) / 100.0;
    if (m_savings != newSavings) {
        m_savings = newSavings;
        emit savingsChanged();
    }
}
