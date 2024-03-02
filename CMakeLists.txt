cmake_minimum_required(VERSION 3.16)

project(SimpleAI VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTORCC ON)
set(Qt6_DIR /home/malte/Qt6/6.7.0/gcc_64/lib/cmake/Qt6/)
set(Qt6Quick_DIR /home/malte/Qt6/6.7.0/gcc_64/lib/cmake/Qt6Quick/)
# set(Qt6_DIR /home/malte/Qt6/6.7.0/gcc_64/lib/cmake/Qt6/)

add_subdirectory(external/simpleClai)
add_subdirectory(src/modules/models)

find_package(Qt6 6.5 REQUIRED COMPONENTS Quick)

#set(QML_IMPORT_PATH "${CMAKE_CURRENT_BINARY_DIR}/src/modules/models")

qt_standard_project_setup(REQUIRES 6.5)

qt_add_executable(appSimpleAI
    src/main.cpp
    include/code/WindowManager.hpp
    src/code/WindowManager.cpp
    include/code/ClAi.hpp
    src/code/ClAi.cpp
    src/core.qrc
    assets/assets.qrc
)


qt_add_qml_module(appSimpleAI
    URI SimpleAI
    VERSION 1.0
    QML_FILES
        src/design/windows/MainWindow.qml
        src/design/windows/SecondWindow.qml
        src/design/widgets/BigButton.qml
        src/design/config/Colors.qml
        src/design/config/Fonts.qml
        src/design/widgets/SaiScrollView.qml
    RESOURCES
        src/core.qrc
        assets/assets.qrc
    SOURCES
        include/code/ClAi.hpp
        src/code/ClAi.cpp
)

target_include_directories(appSimpleAI PUBLIC
    include/code
)

target_link_libraries(appSimpleAI
    PRIVATE Qt6::Quick
    PUBLIC libsclai SaiModels
)


include(GNUInstallDirs)
install(TARGETS appSimpleAI
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
