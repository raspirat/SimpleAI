import QtQuick
import QtQuick.Controls

ApplicationWindow {
    visible: true
    width: 400
    height: 300
    title: "Main Window"

    Button {
        text: "Open Second Window"
        anchors.centerIn: parent
        onClicked: Qt.application.manager.openWindow("qrc:/design/windows/SecondWindow.qml");
    }
}
