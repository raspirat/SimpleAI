import QtQuick 2.15
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Style 1.0
import Font 1.0

TextField {
    id: control
    property bool isBold: false
    property real radius: 12
    property color borderColor: Style.borderColor
    property color placeholderColor: Style.placeholderColor

    placeholderText: qsTr("...")
    placeholderTextColor: placeholderColor

    font.pixelSize: Font.h3
    font.family: isBold ? Font.getContentFontBold.name : Font.getContentFont.name
    font.bold: isBold ? Font.Bold : Font.Normal
    font.weight: isBold ? Font.Bold : Font.Normal

    color: Style.textColor

    background: Rectangle {
        height: control.height
        width: control.width
        radius: control.radius
        color: Style.popupBackground
        border.width: control.activeFocus ? 2 : 1
        border.color: control.borderColor
    }
}
