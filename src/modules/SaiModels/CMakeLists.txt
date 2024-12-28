cmake_minimum_required(VERSION 3.16)

project(SaiModels VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTORCC ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Quick)

qt_standard_project_setup(REQUIRES 6.5)

qt_add_library(SaiModels STATIC)

qt_add_qml_module(SaiModels
    URI "SaiModels"
    VERSION 1.0
    SOURCES
        JsonModel.hpp
        JsonModel.cpp
    RESOURCE_PREFIX /
)

