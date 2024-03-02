cmake_minimum_required(VERSION 3.16)

project(SaiModels VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTORCC ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Quick)

qt_standard_project_setup(REQUIRES 6.5)

add_library(SaiModels
    include/JsonModel.hpp
    src/JsonModel.cpp
)

qt_add_qml_module(SaiModels
    URI JsonModel
    VERSION 1.0
    PLUGIN_TARGET SaiModelsPlugin
    SOURCES
        include/JsonModel.hpp
        src/JsonModel.cpp
)

set(QML_IMPORT_PATH "../${CMAKE_CURRENT_BINARY_DIR}")

target_include_directories(SaiModels PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)
