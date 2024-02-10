import QtQuick 2.15
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3
import Style 1.0
import Font 1.0

ComboBox {
    id: control

    property real radius: 12
    property color backgroundColor: "#2a2d36"

    property color checkedColor: Style.popupBackground


    delegate: ItemDelegate{
        id: itemDelegate
        width: control.implicitWidth * 1.2
        height: control.implicitHeight / 1.2
        hoverEnabled: true
        focus: true

        background: Rectangle{
            anchors.fill: parent
            radius: 8
            color: itemDelegate.hovered ? control.backgroundColor : "transparent"
        }

        RowLayout{
            Layout.alignment: Qt.AlignVCenter
            width: parent.width
            height: parent.height
            anchors.fill: parent
            spacing: 10
            Layout.leftMargin: 10
            Layout.rightMargin: 10

            Label{
                opacity: 0.9
                text: modelData
                font.pixelSize: Font.h3
                color: itemDelegate.hovered ? "#FFFFFF" : Style.textColor
                Layout.fillWidth: true
                verticalAlignment: Image.AlignVCenter
                Layout.alignment: Qt.AlignVCenter
                Layout.leftMargin: 10
            }
        }
    }

    contentItem: Item{
        width: control.background.width - control.indicator.width - 10
        height: control.background.height
        RowLayout{
            anchors.fill: parent
            spacing: 10
            Label{
                opacity: 0.9
                text: control.displayText
                font.pixelSize: Font.h3
                Layout.fillWidth: true
                verticalAlignment: Image.AlignVCenter
                Layout.alignment: Qt.AlignVCenter
                Layout.leftMargin: 10
                color: "#FFFFFF"
            }
        }
    }


    background: Rectangle{
        implicitWidth: control.implicitWidth
        implicitHeight: control.implicitHeight
        color: control.down ? Qt.darker(control.checkedColor,1.2) : control.checkedColor
        radius: control.radius
        border.width: control.activeFocus ? 2 : 0.6
        border.color: Style.borderColor
    }

    popup: Popup{
        y: control.height + 2
        width: control.implicitWidth * 1.26
        implicitHeight: contentItem.implicitHeight > 250 ? 250 : contentItem.implicitHeight
        padding: 4
        contentItem: ListView{
            leftMargin: 5
            implicitHeight: contentHeight
            keyNavigationEnabled: true
            model: control.popup.visible ? control.delegateModel : null
            clip: true
            focus: true
            currentIndex: control.highlightedIndex
        }

        background: Rectangle{
            anchors.fill: parent
            color: Style.popupBackground
            radius: 6
            border.width: 0.6
            border.color: Style.borderColor
            clip: true
        }
    }
}
