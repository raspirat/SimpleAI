import QtQuick 2.9
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    anchors.fill: parent

    ColumnLayout{
        id: columnLayout
        width: parent.width
        height: parent.height / 1.8
        spacing: 20

        Rectangle {
            width: 40
            height: 40
            color: "transparent"
        }

        Image {
            id: logo
            sourceSize.width: parent.width / 6
            source: "qrc:/assets/logo.png"
            fillMode: Image.PreserveAspectFit
            anchors.horizontalCenter: parent.horizontalCenter
        }

        Rectangle {
            width: 50
            height: 50
            color: "transparent"
        }

        Image {
            id: newproject
            sourceSize.width: 400
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

        Image {
            id: newdataset
            sourceSize.width: 400
            source: "qrc:/assets/new-dataset.png"
            fillMode: Image.PreserveAspectFit

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onClicked: {
                   contentFrame.replace("qrc:/NewDataset.qml")
                }
                onEntered: {
                    parent.source = "qrc:/assets/new-dataset-light.png"
                }
                onExited: {
                    parent.source = "qrc:/assets/new-dataset.png"
                }
            }
            Layout.alignment: Qt.AlignHCenter
        }

        Image {
            id: newmodel
            sourceSize.width: 400
            source: "qrc:/assets/new-model.png"
            fillMode: Image.PreserveAspectFit

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onClicked: {
                   contentFrame.replace("qrc:/NewModel.qml")
                }
                onEntered: {
                    parent.source = "qrc:/assets/new-model-light.png"
                }
                onExited: {
                    parent.source = "qrc:/assets/new-model.png"
                }
            }
            Layout.alignment: Qt.AlignHCenter
        }

        Image {
            id: newprofile
            sourceSize.width: 400
            source: "qrc:/assets/new-profile.png"
            fillMode: Image.PreserveAspectFit

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onClicked: {
                   contentFrame.replace("qrc:/NewProfile.qml")
                }
                onEntered: {
                    parent.source = "qrc:/assets/new-profile-light.png"
                }
                onExited: {
                    parent.source = "qrc:/assets/new-profile.png"
                }
            }
            Layout.alignment: Qt.AlignHCenter
        }
    }
}
