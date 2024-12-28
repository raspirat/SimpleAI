import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config

Item {
    property StackView headerView
    property string heading

    implicitHeight: 130
    implicitWidth: parent.width - 50

    Layout.alignment: Qt.AlignCenter | Qt.AlignTop

    Layout.topMargin: 20

    RowLayout {
        implicitHeight: parent.height
        implicitWidth: parent.width

        anchors.fill: parent
        anchors.centerIn: parent

        Buttons.DefaultButton {
            Layout.alignment: Qt.AlignLeft

            implicitHeight: parent.height
            implicitWidth: parent.width / 6

            text: "<"
            font: fonts.altFont
            fontColor: colors.special1
            onClicked: {
                headerView.pop()
            }
        }

        Shapes.NmRect {
            Layout.alignment: Qt.AlignRight

            implicitHeight: parent.height
            implicitWidth: parent.width * 5 / 6 - 20

            Text {
                id: logo
                text: heading
                font.family: fonts.logoFont.family
                font.pixelSize: parent.height - 10
                anchors.centerIn: parent
            }
        }
    }
}
