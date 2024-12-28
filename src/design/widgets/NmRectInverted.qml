import QtQuick
import QtQuick.Templates as T
import Qt5Compat.GraphicalEffects

Item {
    id: nmRect
    width: 200
    height: 100

    property color mainColor: "#E0E5EC"
    property color shadowBrightColor: Qt.lighter(mainColor, 1.05)
    property color shadowDarkColor: Qt.darker(mainColor, 1.1)
    property int cornerRadius: 20

    Rectangle {
        id: content
        width: parent.width
        height: parent.height
        color: mainColor
        radius: cornerRadius
        anchors.centerIn: parent
    }

    InnerShadow {
        horizontalOffset: -5
        verticalOffset: -5
        anchors.fill: content
        radius: 20
        samples: 22
        color: shadowBrightColor
        source: content
    }

    InnerShadow {
        horizontalOffset: 5
        verticalOffset: 5
        anchors.fill: content
        radius: 20
        spread: -1
        samples: 22
        color: shadowDarkColor
        source: content
    }



}
