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

    property StackView stackView
    property string name

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
            id: siteHeader
            heading: name
            headerView: stackView
        }

        Items.SaiCdSelector {
            id: cdSelector
        }

        Items.SaiScrollView {
            id: scrollView
            description: "recent " + name + ":"
            file: ""
        }
    }
}
