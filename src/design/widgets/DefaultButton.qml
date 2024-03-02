import QtQuick
import QtQuick.Controls
import QtQuick.Templates as T
import Qt5Compat.GraphicalEffects
import QtQuick.Layouts

import "qrc:/shapes" as Shapes
import "qrc:/config" as Config

T.Button {
    id: defButton
    height: 150
    width: 150
    text: "DefaultButton"

    Config.Fonts {
        id: fonts
    }

    property color fontColor: "#A3B1C6"
    property color rectMainColor: "#E0E5EC"
    property int cornerRadius: 20
    property int pixelSize: height - 10
    property font descFont: fonts.mainFont


    Behavior on scale {
        NumberAnimation {
            duration: 200
        }
    }

    Shapes.NmRect {
        id: backRect
        width: parent.width
        height: parent.height
        scale: parent.scale
        mainColor: rectMainColor
        cornerRadius: cornerRadius
    }

    Shapes.NmRectInverted {
        id: backRectI
        width: parent.width
        height: parent.height
        scale: parent.scale
        mainColor: rectMainColor
        cornerRadius: cornerRadius
        visible: false
    }

    ColumnLayout {
        anchors.fill: parent

        Text {
            id: mainText
            text: defButton.text
            opacity: enabled ? 1.0 : 0.3
            color: fontColor
            font.family: defButton.font.family
            font.pixelSize: pixelSize
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            elide: Text.ElideRight
            smooth: true
            Layout.alignment: Qt.AlignCenter
        }
    }


    MouseArea {
        id: area
        anchors.fill: parent
        hoverEnabled: true
        onEntered: {
            defButton.scale = 1.03
        }
        onExited: {
            defButton.scale = 1
        }

        onPressedChanged: {
            backRectI.visible = area.pressed
        }

        onReleased: {
            backRectI.visible = false
            defButton.released()
        }

        onClicked: {
            defButton.clicked()
        }
    }
}
