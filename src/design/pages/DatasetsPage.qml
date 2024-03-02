import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config
import "qrc:/items" as Items

Item
{
    Config.Colors {id: colors}
    Config.Fonts {id: fonts}

    property StackView view

    height: parent.height
    width: parent.width

    Items.SaiMainPage {
        name: "Datasets"
        stackView: view
    }
}
