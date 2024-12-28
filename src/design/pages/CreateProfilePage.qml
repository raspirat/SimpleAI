import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QtQuick.Dialogs

import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/config" as Config
import "qrc:/items" as Items

Rectangle  {
    property StackView stackView

    Config.Fonts {id: fonts}
    Config.Colors {id: colors}

    color: colors.back

    ColumnLayout
    {
        anchors.fill: parent
        anchors.horizontalCenter: parent.horizontalCenter
        height: parent.height / 3
        width: parent.width
        spacing: 10

        Items.SaiSiteHeader {
            id: createSiteHeader
            heading: qsTr("Create...")
            headerView: stackView
        }

        Text {
            id: caption
            text: qsTr("Please fill in the parameters")
            color: colors.secondaryFont
            font: fonts.mainFont
            scale: 2
            Layout.alignment: Qt.AlignHCenter
        }

        Items.Textbox {
            id: profileName
            placeholder: "Name"
            implicitWidth: parent.width - 50
            Layout.alignment: Qt.AlignHCenter
        }

        Items.Textbox {
            id: framework
            Layout.alignment: Qt.AlignHCenter
            placeholder: "Framework"
            implicitWidth: parent.width - 50
        }

        Items.Textbox {
            id: scope
            Layout.alignment: Qt.AlignHCenter
            placeholder: "Scope"
            implicitWidth: parent.width - 50
        }

        Rectangle {
            color: "transparent"
            width: parent.width
            height: parent.height / 15
        }

        Buttons.BigButton {
            Layout.alignment: Qt.AlignHCenter
            text: "Done"
            description: ""
            font: fonts.mainFont
            implicitWidth: parent.width - 50
            implicitHeight: 100
            fontColor: colors.special2
            onClicked: {
                if (profileName.text !== "" && scope.text !== "" && framework.text !== "") {
                    let retCode = ClAi.createProfile(profileName.text, framework.text, scope.text);
                } else {
                    if (profileName.text === "") {profileName.placeholderTextColor = colors.danger;}
                    else {datasetName.placeholderTextColor = colors.settings;}
                    if (scope.text === "") {scope.placeholderTextColor = colors.danger;}
                    else {labelsPath.placeholderTextColor = colors.settings;}
                    if (framework.text === "") {framework.placeholderTextColor = colors.danger;}
                    else {dataPath.placeholderTextColor = colors.settings;}
                }
            }
        }

        Popup {
            id: deleteFeedback
            anchors.centerIn: Overlay.overlay
            width: 300
            height: 200
            modal: true
            focus: true
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
            background: Shapes.NmRect {
                implicitHeight: parent.height
                implicitWidth: parent.width
                anchors.fill: parent
            }

            // Seppi, mach du das Popup, danke!
        }
    }
}
