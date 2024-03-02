import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config
import "qrc:/items" as Items

Rectangle
{
    Config.Colors {id: colors}
    Config.Fonts {id: fonts}

    property StackView createStackView
    property string createName

    color: colors.back

    height: parent.height
    width: parent.width


    ColumnLayout
    {
        anchors.fill: parent
        anchors.centerIn: parent

        height: parent.height
        width: parent.width


        Items.SaiSiteHeader {
            id: createSiteHeader
            heading: createName
            headerView: createStackView
        }
    }
}
