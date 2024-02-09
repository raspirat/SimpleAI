import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 2.15

Item {
    anchors.fill: parent

    ColumnLayout {
        spacing: 20
        anchors.horizontalCenter: parent.horizontalCenter

        Rectangle {
            width: 40
            height: 40
            color: "transparent"
        }

        Text {
            id: newprojectcaption
            text: qsTr("Create a new project")
            font.pixelSize: 50
            color: "aqua"
            anchors.horizontalCenter: parent.horizontalCenter
            font.family: "Rubik"
        }

        Text {
            id: description
            text: qsTr('<html><style type="text/css">a{color: "#4287f5"}</style>Here you can create a model. Therefore you need a dataset as well as a profile. If you currently don\'t have them or don\'t know what to do, take a look at the GitHub page of <a href="https://github.com/sertschgi/simpleClai">SimpleClai</a>.</html>')
            onLinkActivated: Qt.openUrlExternally("https://github.com/sertschgi/simpleClai")
            color: "#A1A1A1"
            font.pixelSize: 20
            wrapMode: Wrap
            anchors.horizontalCenter: newprojectcaption.horizontalCenter
            width: newprojectcaption.contentWidth * 1.5
            height: 100
            Rectangle {
                border.color: "red"
                color: "transparent"
                width: parent.contentWidth
                height: parent.contentHeight
            }
        }

        Rectangle {
            id: nameRect
            width: 300
            height: 40

            anchors.horizontalCenter: parent.horizontalCenter

            Image {
                source: "qrc:/assets/type.png"
                anchors.fill: parent

                TextInput {
                    id: textInput
                    anchors.fill: parent
                    anchors.margins: 10
                    font.pixelSize: 16
                    color: "black"
                }

                Text {
                    anchors.left: parent.left
                    anchors.leftMargin: 10
                    anchors.verticalCenter: parent.verticalCenter
                    text: "Name"
                    color: "#888888"
                    font.pixelSize: 16
                    visible: textInput.text.length === 0
                }
            }
        }
    }
}
