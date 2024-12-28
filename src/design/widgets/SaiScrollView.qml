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
                    id: list
                    implicitHeight: parent.height
                    implicitWidth: parent.width

                    snapMode: ListView.SnapToItem

                    model: JsonModel {
                        id: jsonModel
                    }

                    Component.onCompleted: {
                        var json = ClAi.getDatasetsJson();
                        jsonModel.init(json);
                    }

                    delegate: Control {
                        property bool isSelected: false // Track whether this item is selected

                        implicitHeight: container.height / 5
                        implicitWidth: container.width - 30

                        topPadding: 10
                        bottomPadding: 10
                        leftPadding: 20

                        contentItem: Rectangle {
                            id: delegateRect
                            implicitHeight: parent.height
                            implicitWidth: parent.width

                            color: isSelected ? colors.special1 : (delegateRect.containsMouse ? colors.special2 : colors.special1)

                            radius: 15

                            Text {
                                text: key
                                font.family: fonts.mainFont.family
                                anchors.centerIn: parent
                            }
                        }

                        MouseArea {
                            anchors.fill: parent
                            hoverEnabled: true

                            onClicked: {
                                // Deselect all other items
                                for (var i = 0; i < list.model.count; ++i) {
                                    if (i !== index) {
                                        list.itemAtIndex(i).isSelected = false;
                                    }
                                }
                                isSelected = true; // Select this item
                            }

                            /*onEntered: {
                                if (!isSelected) {
                                    delegateRect.color = colors.special2; // Change color on hover
                                }
                            }

                            onExited: {
                                if (!isSelected) {
                                    delegateRect.color = list.currentIndex === index ? colors.special2 : colors.special1; // Revert color on exit
                                }
                            }*/
                        }
                    }
                }
            }
        }
    }
}
