import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config
import "qrc:/items" as Items

Rectangle
{
    property StackView createStackView
    property string createName

    Config.Colors {id: colors}
    Config.Fonts {id: fonts}

    color: colors.back

    width: parent.width
    height: parent.height

    ColumnLayout
    {
        anchors.fill: parent
        anchors.horizontalCenter: parent.horizontalCenter
        height: parent.height / 3
        width: parent.width
        spacing: 10

        Items.SaiSiteHeader {
            id: createSiteHeader
            heading: qsTr("Create...")
            headerView: createStackView
        }

        Text {
            id: caption
            text: qsTr("Please fill in the parameters")
            color: colors.secondaryFont
            font: fonts.mainFont
            scale: 2
            Layout.alignment: Qt.AlignHCenter
        }

        Items.Textbox {
            id: nameBox
            implicitWidth: parent.width - 50
            Layout.alignment: Qt.AlignHCenter
            placeholder: qsTr("Name")
        }

        Rectangle {
            width: parent.width
            height: parent.height - 600
            color: "transparent"
        }
    }
}

