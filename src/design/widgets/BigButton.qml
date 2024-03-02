import QtQuick
import QtQuick.Controls
import QtQuick.Templates as T
import Qt5Compat.GraphicalEffects
import QtQuick.Layouts

import "qrc:/shapes" as Shapes
import "qrc:/config" as Config

T.Button {
    id: bigButton
    implicitHeight: 150
    implicitWidth: 150
    text: "BigButton"

    Config.Fonts {
        id: fonts
    }

    property color fontColor: "#A3B1C6"
    property color mainColor: "#E0E5EC"
    property int cornerRadius: 20
    property int pixelSize: height - 50
    property string description: "a big Button"
    property font descFont: fonts.mainFont


    Shapes.NmRect {
        id: backRect
        width: bigButton.width
        height: bigButton.height
        mainColor: mainColor
        cornerRadius: cornerRadius
    }

    Shapes.NmRectInverted {
        id: backRectI
        width: bigButton.width
        height: bigButton.height
        mainColor: mainColor
        cornerRadius: cornerRadius
        visible: false
    }

    ColumnLayout {
        anchors.fill: parent

        Text {
            id: mainText
            text: bigButton.text
            opacity: enabled ? 1.0 : 0.3
            color: fontColor
            font.family: bigButton.font.family
            font.pixelSize: pixelSize
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            elide: Text.ElideRight
            smooth: true
            Layout.alignment: Qt.AlignCenter

            Behavior on bottomPadding {
                NumberAnimation {
                    duration: 400
                }
            }
        }

        Text {
            id: buttonDesc
            text: description
            font.family: descFont.family
            Layout.alignment: Qt.AlignCenter
            opacity: 0
            visible: false
            bottomPadding: 0
            topPadding: -30

            Behavior on opacity {
                NumberAnimation {
                    duration: 500
                }
            }
        }
    }


    MouseArea {
        id: area
        anchors.fill: parent
        hoverEnabled: true
        onEntered: {
            buttonDesc.opacity = 1
            mainText.bottomPadding = 10
            buttonDesc.visible = true
        }
        onExited: {
            buttonDesc.opacity = 0
            mainText.bottomPadding = 0
            if (mainText.bottomPadding === 0)
            {
                buttonDesc.visible = false
            }
        }

        onPressedChanged: {
            backRectI.visible = area.pressed
        }

        onReleased: {
            backRectI.visible = false
            bigButton.released()
        }

        onClicked: {
            bigButton.clicked()
        }
    }
}
