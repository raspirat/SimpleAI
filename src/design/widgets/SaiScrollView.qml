import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

// import JsonModel

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config


Item {
    Config.Fonts {id: fonts}

    property var json
    property string description: "a scroll view:"

    Layout.alignment: Qt.AlignCenter

    implicitHeight: parent.height - 310
    implicitWidth: parent.width - 50

    ColumnLayout
    {
        height: parent.height
        width: parent.width

        Text {
            Layout.alignment: Qt.AlignLeft
            width: parent.width
            height: 60
            text: description
            font.family: fonts.altFont.family
            font.pointSize: 30
        }

        Shapes.NmRect {
            id: container

            implicitHeight: parent.height - 60
            implicitWidth: parent.width
            /*
            ScrollView {
                ListView {
                    model: JsonModel {id: jsonModel}

                    Component.onCompleted: {
                        var jsonModels = json;
                        jsonModel.init(jsonModels);
                    }

                    delegate: Rectangle {
                        implicitHeight: container.height / 10
                        implicitWidth: container.width

                        Text {
                            text: key
                            font.family: descFont.family
                            anchors.centerIn: parent
                        }
                    }
                }
            }*/
        }
    }
}
