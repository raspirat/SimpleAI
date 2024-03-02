import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import "qrc:/config" as Config
import "qrc:/buttons" as Buttons
import "qrc:/shapes" as Shapes
import "qrc:/pages/MainWindow" as Pages


Window {
    Config.Fonts {
        id: fonts
    }

    Config.Colors {
        id: colors
    }

    width: 640
    height: 900
    //maximumHeight: 900
    //minimumHeight: 900
    //maximumWidth: 640
    //minimumWidth: 640

    visible: true
    title: qsTr("Hello World")
    color: "#E0E5EC";



    StackView {
        id: stackView
        anchors.fill: parent

        initialItem: mainWindowPageComponent

        Component
        {
            id: mainWindowPageComponent
            Pages.MainWindowPage {
                view: stackView
            }
        }

        Component {
            id: datasetsPageComponent
            Pages.DatasetsPage {
                view: stackView
            }
        }

        Component {
            id: profilesPageComponent
            Pages.ProfilesPage {
                view: stackView
            }
        }

        Component {
            id: projectsPageComponent
            Pages.ProjectsPage {
                view: stackView
            }
        }

        Component {
            id: modelsPageComponent
            Pages.ModelsPage {
                view: stackView
            }
        }

        Component {
            id: trainPageComponent
            Pages.TrainPage {
                view: stackView
            }
        }
    }
}
