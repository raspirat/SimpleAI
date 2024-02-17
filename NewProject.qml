import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 2.15
import "./components"
import Style 1.0

Item {
    anchors.fill: parent

    ColumnLayout {
        anchors.horizontalCenter: parent.horizontalCenter
        spacing: 30

        Rectangle {
            Layout.fillWidth: true
            height: 100
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
            text: qsTr('Here you can create a project. Therefore you need a dataset as well as a profile. If you currently don\'t have them or don\'t know what to do, take a look at the GitHub page of <a href="https://github.com/sertschgi/simpleClai">SimpleClai</a>.')
            onLinkActivated: Qt.openUrlExternally("https://github.com/sertschgi/simpleClai")
            linkColor: "#4287f5"
            color: Style.textColor
            font.pixelSize: 20
            wrapMode: Text.Wrap
            anchors.horizontalCenter: newprojectcaption.horizontalCenter
            width: newprojectcaption.contentWidth * 1.5
        }

        // TODO: Space before TextInput
        CustomTextField {
            id: nameRect
            implicitWidth: 350
            implicitHeight: 50
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
            implicitWidth: 350
            implicitHeight: 50
        }

        Dropdown {
            model: ["Please select a dataset...", "Dataset 1", "Dataset 2"]
            anchors.horizontalCenter: parent.horizontalCenter
            implicitWidth: 350
            implicitHeight: 50
        }

        Rectangle {
            color: "transparent"
            Layout.fillWidth: true
            height: 25
        }

        Popup {
            id: newProjectFeedback
            anchors.centerIn: Overlay.overlay
            width: 300
            height: 200
            modal: true
            focus: true
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
            onClosed: {
                contentFrame.replace("qrc:/MainMenu.qml", StackView.PopTransition)
            }

            Image {
                id: popupImg
                anchors.fill: parent
                fillMode: Image.PreserveAspectFit

                Image {
                    id: popupClose
                    width: 20
                    height: 20
                    source: "qrc:/assets/close.png"
                    anchors.top: parent.top
                    anchors.right: parent.right

                    MouseArea {
                        anchors.fill: parent
                        hoverEnabled: true
                        onClicked: {
                            newProjectFeedback.close()
                        }
                        onEntered: {
                            parent.source = "qrc:/assets/close-red.png"
                        }
                        onExited: {
                            parent.source = "qrc:/assets/close.png"
                        }
                    }
                }
            }
        }

        Image {
            id: createNewProject
            sourceSize.width: 700
            source: "qrc:/assets/create.png"
            fillMode: Image.PreserveAspectFit

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onClicked: {
                    // CHECK STUFF ETC!!!
                    // CALLBACK!!!!!!!!!!
                    newProjectFeedback.open()
                }
                onEntered: {
                    parent.source = "qrc:/assets/create-light.png"
                }
                onExited: {
                    parent.source = "qrc:/assets/create.png"
                }
            }
            Layout.alignment: Qt.AlignHCenter
        }
    }
}
