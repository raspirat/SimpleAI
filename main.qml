import QtQuick 2.9
import QtQuick.Controls 2.5

ApplicationWindow {
    width: 1920
    height: 1080
    visibility: "Maximized"
    visible: true
    title: qsTr("Hello World")

    StackView {
        id: contentFrame
        anchors.fill: parent
        width: parent.width
        height: parent.height
        initialItem: Qt.resolvedUrl("qrc:/MainMenu.qml")
    }
}
