import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import SaiModels

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config


Item {
    id: sv

    Config.Fonts {id: fonts}
    Config.Colors {id: colors}

    property var json
    property string description: "a scroll view:"

    Layout.alignment: Qt.AlignCenter

    implicitHeight: parent.height - 310
    implicitWidth: parent.width - 50

    ColumnLayout
    {
        implicitHeight: parent.height
        implicitWidth: parent.width

        Text {
            Layout.alignment: Qt.AlignLeft
            width: parent.width
            height: parent.height
            text: description
            font.family: fonts.altFont.family
            font.pointSize: 30
        }

        Shapes.NmRect {
            id: container

            implicitHeight: sv.height - 60
            implicitWidth: sv.width

            ScrollView {
                implicitHeight: parent.height
                implicitWidth: parent.width


                ListView {
                    implicitHeight: parent.height
                    implicitWidth: parent.width

                    model: JsonModel {
                        id: jsonModel
                    }

                    Component.onCompleted: {
                        var json = ClAi.getDatasetsJson();
                        console.log(json);
                        jsonModel.init(json);
                    }

                    delegate: Control {
                        implicitHeight: container.height / 5
                        implicitWidth: container.width - 30

                        topPadding: 10
                        bottomPadding: 10
                        leftPadding: 20

                        // anchors.centerIn: parent

                        contentItem: Rectangle {
                            implicitHeight: parent.height
                            implicitWidth: parent.width

                            color: colors.special1

                            radius: 20

                            Text {
                                text: key
                                font.family: fonts.mainFont.family
                                anchors.centerIn: parent
                            }
                        }
                    }
                }
            }
        }
    }
}
