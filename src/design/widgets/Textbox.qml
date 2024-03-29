import QtQuick
import QtQuick.Controls 2.15
import QtQuick.Templates as T
import Qt5Compat.GraphicalEffects
import QtQuick.Layouts

import "qrc:/shapes" as Shapes
import "qrc:/config" as Config

TextField {
    id: textBox
    implicitHeight: 100
    implicitWidth: 500

    Config.Fonts {
        id: fonts
    }

    Config.Colors {
        id: colors
    }

    property color fontColor: "#A3B1C6"
    property color mainColor: "#E0E5EC"
    property int cornerRadius: 20
    property int pixelSize: height - 50
    property string placeholder: "Textbox"
    property font descFont: fonts.mainFont

    background: Shapes.NmRect {
        id: backRect
        width: textBox.width
        height: textBox.height
        mainColor: mainColor
        cornerRadius: cornerRadius
    }

    placeholderText: qsTr(placeholder)
    placeholderTextColor: colors.settings
    font.pixelSize: pixelSize

    color: colors.mainFont

    Behavior on placeholderTextColor {
        ColorAnimation {
            duration: 100
        }
    }
}
