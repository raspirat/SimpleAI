import QtQuick 2.9
import QtQuick.Controls 2.5
import QtQuick.Window 2.9
import QtQuick.Layouts 2.9

ApplicationWindow {
    id: mainWindow
    width: 1920
    height: 1080
    visibility: "Maximized"
    visible: true
    title: qsTr("SimpleAI")

    Rectangle {
        anchors.fill: parent
        color: "#2D2D2D"
    }

    ColumnLayout {
        id: navCol
        anchors.fill: parent
        width: parent.width
        height: parent.height / 10
        anchors.horizontalCenter: parent.horizontalCenter

        RowLayout{
            width: parent.width
            anchors.horizontalCenter: parent.horizontalCenter

            Image {
                id: home
                sourceSize.width: mainWindow.width / 40
                source: "qrc:/assets/home.png"
                fillMode: Image.PreserveAspectFit
                anchors.horizontalCenter: parent.horizontalCenter
            }
        }

        Rectangle {
            border.color: "white"
            color: "white"
            width: parent.contentWidth
            height: 200
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
