import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "qrc:/config" as Config
import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes


ColumnLayout
{
    Config.Fonts {
        id: fonts
    }

    Config.Colors {
        id: colors
    }

    property StackView view

    Shapes.NmRect {
        Layout.alignment: Qt.AlignCenter
        height: 130
        width: 600

        Text {
            id: logo
            text: "SimpleAi"
            font: fonts.logoFont
            scale: 10
            anchors.centerIn: parent
        }
    }

    Text {
        text: "choose..."
        color: colors.secondaryFont
        font: fonts.mainFont
        topPadding: 60
        scale: 2
        Layout.alignment: Qt.AlignCenter
    }


    GridLayout {
        id: grid
        flow: GridLayout.TopToBottom
        columns: 3
        Layout.alignment: Qt.AlignCenter
        columnSpacing: 30
        rowSpacing: 30
        implicitHeight: 600
        implicitWidth: 600

        Buttons.BigButton {
            text: "D"
            description: "datasets"
            font: fonts.altFont
            Layout.row: 0
            Layout.column: 0
            fontColor: colors.special1
            onClicked: {
                stackView.push(datasetsPageComponent)
            }
        }

        Buttons.BigButton {
            text: "Pf"
            description: "profiles"
            font: fonts.altFont
            Layout.row: 0
            Layout.column: 1
            fontColor: colors.special1
            onClicked: {
                stackView.push(profilesPageComponent)
            }
        }

        Buttons.BigButton {
            text: "Pj"
            description: "projects"
            font: fonts.altFont
            Layout.row: 0
            Layout.column: 2
            fontColor: colors.special1
            onClicked: {
                stackView.push(projectsPageComponent)
            }
        }

        Buttons.BigButton {
            text: "M"
            description: "models"
            font: fonts.altFont
            Layout.row: 1
            Layout.column: 0
            fontColor: colors.special1
            onClicked: {
                stackView.push(modelsPageComponent)
            }
        }

        Buttons.BigButton {
            text: "T"
            description: "train"
            font: fonts.altFont
            Layout.row: 1
            Layout.column: 1
            fontColor: colors.special1
            onClicked: {
                stackView.push(trainPageComponent)
            }
        }

        Buttons.BigButton {
            text: "B"
            description: "builder"
            font: fonts.altFont
            Layout.row: 1
            Layout.column: 2
            fontColor: colors.special2
            onClicked: {
                WindowManager.switchWindows("qrc:/design/windows/SecondWindow.qml");
            }
        }

        Buttons.BigButton {
            text: "L"
            description: "labeler"
            font: fonts.altFont
            Layout.row: 2
            Layout.column: 0
            fontColor: colors.special2
            onClicked: {
                WindowManager.switchWindows("qrc:/design/windows/SecondWindow.qml");
            }
        }

        Buttons.BigButton {
            text: "M"
            description: "marketplace"
            font: fonts.altFont
            Layout.row: 2
            Layout.column: 1
            fontColor: colors.special2
            onClicked: {
                WindowManager.switchWindows("qrc:/design/windows/SecondWindow.qml");
            }
        }

        Buttons.BigButton {
            text: "S"
            description: "settings"
            font: fonts.altFont
            Layout.row: 2
            Layout.column: 2
            fontColor: colors.special2
            onClicked: {
                WindowManager.switchWindows("qrc:/design/windows/SecondWindow.qml");
            }
        }
    }
}
