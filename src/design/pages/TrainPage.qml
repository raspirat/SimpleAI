import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config
import "qrc:/items" as Items

Rectangle
{
    Config.Colors {id: colors}

    property StackView view

    color: colors.back

    ColumnLayout
    {
        anchors.fill: parent
        anchors.centerIn: parent

        implicitHeight: parent.height
        implicitWidth: parent.width

        Items.SaiSiteHeader {
            id: siteHeader
            heading: "Training"
            headerView: view
        }

        Item {
            Layout.alignment: Qt.AlignCenter

            implicitHeight: parent.height - siteHeader.height - 50
            implicitWidth: parent.width - 50

            PathView {
                id: pathView
                model: 3
                preferredHighlightBegin: 0.5
                preferredHighlightEnd: 0.5
                pathItemCount: 3
                highlightRangeMode: PathView.StrictlyEnforceRange

                implicitHeight: parent.height
                implicitWidth: parent.width

                anchors.fill: parent
                anchors.centerIn: parent

                path: Path {
                    startX: 0; startY: 0;
                    PathLine {x: 0}
                    PathLine {x: 100}
                    PathLine {x: 200}
                }

                delegate: Loader {
                    //implicitWidth: parent.width
                    //implicitHeight: parent.height

                    anchors.fill: parent

                    sourceComponent: {
                        svProjectSelect
                    }

                    Component {
                        id: svProjectSelect

                        Items.SaiScrollView {
                            implicitHeight: pathView.height
                            implicitWidth: pathView.width
                            description: "Select A Project:"
                        }
                    }

                    Component {
                        id: svModelSelect

                        Items.SaiScrollView {
                            implicitHeight: pathView.height
                            implicitWidth: pathView.width
                            description: "Select A Model:"
                            json: ClAi.getProjectsJson()
                        }
                    }
                }
            }
        }
    }
}
