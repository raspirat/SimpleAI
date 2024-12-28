import QtQuick
import QtQuick.Controls

Window {
    width: 1920
    height: 1080
    visible: true
    title: qsTr("Second Window")

    Button {
        text: "other Page"
        anchors.centerIn: parent
        onClicked: {
            WindowManager.switchWindows("qrc:/design/windows/MainWindow.qml");
        }
    }
}
