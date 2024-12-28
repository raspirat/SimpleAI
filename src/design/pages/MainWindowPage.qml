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
        text: "Choose..."
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
            description: "Datasets"
            font: fonts.altFont
            Layout.row: 0
            Layout.column: 0
            fontColor: colors.special1
            onClicked: {
                stackView.push(Qt.resolvedUrl("qrc:/pages/MainWindow/DatasetsPage.qml"), {view: view})
            }
        }

        Buttons.BigButton {
            text: "Pf"
            description: "Profiles"
            font: fonts.altFont
            Layout.row: 0
            Layout.column: 1
            fontColor: colors.special1
            onClicked: {
                stackView.push(Qt.resolvedUrl("qrc:/pages/MainWindow/ProfilesPage.qml"), {view: view})
            }
        }

        Buttons.BigButton {
            text: "Pj"
            description: "Projects"
            font: fonts.altFont
            Layout.row: 0
            Layout.column: 2
            fontColor: colors.special1
            onClicked: {
                stackView.push(Qt.resolvedUrl("qrc:/pages/MainWindow/ProjectsPage.qml"), {view: view})
            }
        }

        Buttons.BigButton {
            text: "Mo"
            description: "Models"
            font: fonts.altFont
            Layout.row: 1
            Layout.column: 0
            fontColor: colors.special1
            onClicked: {
                stackView.push(Qt.resolvedUrl("qrc:/pages/MainWindow/ModelsPage.qml"), {view: view})
            }
        }

        Buttons.BigButton {
            text: "B"
            description: "Builder"
            font: fonts.altFont
            Layout.row: 1
            Layout.column: 1
            fontColor: colors.special2
            onClicked: {
                WindowManager.switchWindows("qrc:/design/windows/SecondWindow.qml");
            }
        }

        Buttons.BigButton {
            text: "T"
            description: "Train"
            font: fonts.altFont
            Layout.row: 1
            Layout.column: 2
            fontColor: colors.special1
            onClicked: {
                stackView.push(Qt.resolvedUrl("qrc:/pages/MainWindow/TrainPage.qml"), {view: view})
            }
        }

        Buttons.BigButton {
            text: "L"
            description: "Labeler"
            font: fonts.altFont
            Layout.row: 2
            Layout.column: 0
            fontColor: colors.special2
            onClicked: {
                WindowManager.switchWindows("qrc:/design/windows/SecondWindow.qml");
            }
        }

        Buttons.BigButton {
            text: "Ma"
            description: "Marketplace"
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
            description: "Settings"
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
