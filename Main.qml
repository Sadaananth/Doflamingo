import QtQuick
import QtQuick.Controls

ApplicationWindow {
    visible: true
    width: 400
    height: 600
    title: "Savings Calculator"

    Column {
        spacing: 16
        padding: 20

        TextField {
            id: salaryInput
            placeholderText: "Enter monthly salary"
            inputMethodHints: Qt.ImhFormattedNumbersOnly
            onTextChanged: {
                calculator.salary = parseFloat(text)
            }
        }

        Slider {
            id: percentageSlider
            from: 0
            to: 100
            value: 10
            onValueChanged: {
                calculator.percentage = value
            }
        }

        Text {
            text: "Savings Percentage: " + percentageSlider.value.toFixed(0) + "%"
        }

        Rectangle {
            width: parent.width
            height: 2
            color: "lightgray"
        }

        Text {
            text: "Monthly Savings: $"
        }
        Rectangle {
            width: parent.width
            height: 2
            color: "lightgray"
        }
    }
}
