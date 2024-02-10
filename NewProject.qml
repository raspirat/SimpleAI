import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 2.15
import "./components"
import Style 1.0

Item {
    anchors.fill: parent

    ColumnLayout {
        spacing: 20
        anchors.horizontalCenter: parent.horizontalCenter

        Rectangle {
            Layout.fillWidth: true
            height: 60
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
            wrapMode: Text.Wrap
            anchors.horizontalCenter: newprojectcaption.horizontalCenter
            width: newprojectcaption.contentWidth * 1.5

            Rectangle {
                border.color: "red"
                color: "transparent"
                width: parent.contentWidth
                height: parent.contentHeight
            }
        }

        // TODO: Space before TextInput
        CustomTextField {
            id: nameRect
            implicitWidth: 300
            implicitHeight: 40
            isBold: false
            placeholderText: qsTr("Name")
            selectedTextColor: "#FFFFFF"
            selectionColor: "lightblue"
            radius: 8
            anchors.horizontalCenter: parent.horizontalCenter
        }

        Dropdown {
            model: ["Please select a profile...", "Profile 1", "Profile 2"]
            anchors.horizontalCenter: parent.horizontalCenter
            implicitWidth: 300
            implicitHeight: 40
        }

        Dropdown {
            model: ["Please select a dataset...", "Dataset 1", "Dataset 2"]
            anchors.horizontalCenter: parent.horizontalCenter
            implicitWidth: 300
            implicitHeight: 40
        }

        Rectangle {
            color: "transparent"
            Layout.fillWidth: true
            height: 35
        }

        Image {
            id: createNewProject
            sourceSize.width: 650
            source: "qrc:/assets/new-project.png"
            fillMode: Image.PreserveAspectFit

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onClicked: {
                   contentFrame.replace("qrc:/NewProject.qml")
                }
                onEntered: {
                    parent.source = "qrc:/assets/new-project-light.png"
                }
                onExited: {
                    parent.source = "qrc:/assets/new-project.png"
                }
            }
            Layout.alignment: Qt.AlignHCenter
        }
    }
}
