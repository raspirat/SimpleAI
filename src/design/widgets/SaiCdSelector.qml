import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config


Item {
    Config.Fonts {id: fonts}
    Config.Colors {id: colors}

    property StackView headerView
    property string heading

    Layout.alignment: Qt.AlignCenter

    height: 80
    width: parent.width - 50

    RowLayout {
        anchors.fill: parent

        height: parent.height
        width: parent.width

        Buttons.DefaultButton {
            Layout.alignment: Qt.AlignLeft

            id: settingsButton

            height: parent.height
            width: parent.width / 6

            text: "*"
            pixelSize: parent.height - 10
            font: fonts.altFont
            fontColor: colors.settings
            onClicked: {
            }
        }

        Item {
            Layout.alignment: Qt.AlignRight

            width: parent.width * 5 / 6 - 20
            height: parent.height

            RowLayout {
                anchors.fill: parent

                Buttons.DefaultButton {
                    Layout.alignment: Qt.AlignLeft

                    id: createButton
                    height: parent.height
                    width: parent.width / 2 - 10
                    text: "+"
                    font.family: fonts.altFont.family
                    fontColor: colors.creation
                    onClicked: {
                        headerView.push(Qt.resolvedUrl("qrc:/items/SaiCreatePage.qml"), {"createName": heading, "createStackView": headerView});
                    }
                }

                Buttons.DefaultButton {
                    Layout.alignment: Qt.AlignRight

                    id: destroyButton
                    height: parent.height
                    width: parent.width / 2 - 10
                    text: "-"
                    font.family: fonts.altFont.family
                    fontColor: colors.destruction
                    onClicked: {
                        deleteFeedback.open()
                    }
                }
            }
        }

        Popup {
            id: deleteFeedback
            anchors.centerIn: Overlay.overlay
            width: 300
            height: 200
            modal: true
            focus: true
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
            background: Shapes.NmRect {
                implicitHeight: parent.height
                implicitWidth: parent.width
                anchors.fill: parent
            }

            // Seppi, mach du das Popup, danke!
        }
    }
}
