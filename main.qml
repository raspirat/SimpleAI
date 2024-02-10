import QtQuick 2.15
import QtQuick.Controls 2.5
import QtQuick.Window 2.9
import QtQuick.Layouts 2.9
import "./components"
import Style 1.0
import QtQuick.Controls.Fusion 2.3

ApplicationWindow {
    id: mainWindow
    minimumWidth: 1920
    minimumHeight: 1080
    visibility: "Maximized"
    visible: true
    title: qsTr("SimpleAI")
    color: Style.background

    MouseArea {
        anchors.fill: parent
        onClicked: forceActiveFocus()
    }

    RowLayout {
        width: parent.width
        height: parent.height / 15
        anchors.horizontalCenter: parent.horizontalCenter

        ColumnLayout {
            anchors.fill: parent
            height: parent.height
            width: parent.width

            Rectangle {
                color: "transparent"
                height: 5
                Layout.fillWidth: true
            }

            Image {
                id: home
                sourceSize.width: mainWindow.width / 35
                source: "qrc:/assets/home.png"
                fillMode: Image.PreserveAspectFit
                anchors.horizontalCenter: parent.horizontalCenter
                MouseArea {
                    anchors.fill: parent
                    hoverEnabled: true
                    onClicked: {
                        if (!String(contentFrame.currentItem).includes("MainMenu")) {
                            contentFrame.replace("qrc:/MainMenu.qml", StackView.PopTransition)
                        }
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                anchors.horizontalCenter: parent.horizontalCenter
                height: 2
                color: "white"
            }
        }
    }

    StackView {
        id: contentFrame
        width: mainWindow.width
        height: mainWindow.height
        anchors.fill: parent
        initialItem: MainMenu {}
    }
}
