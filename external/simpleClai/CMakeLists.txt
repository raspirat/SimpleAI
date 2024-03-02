cmake_minimum_required(VERSION 3.24)

set(EXEC_NAME sclai)
set(LIB_NAME libsclai)

# Set the project name
project(${EXEC_NAME})

# Set the C++ standard to C++17
set(CMAKE_CXX_STANDARD 17)

# Find the Qt5 package
find_package(Qt6 COMPONENTS Core REQUIRED)

set(SOURCE_PATHS
    src/commands/dataset.cpp
    src/commands/model.cpp
    src/commands/profile.cpp
    src/commands/project.cpp
    src/commands/frameworks.cpp
    src/core/parser.cpp
    src/utils/tools.cpp
    src/utils/errors.cpp
)

set(INCLUDE_DIRS
    src/commands
    src/core
    src/utils
    src/config
)

add_executable(${EXEC_NAME}
    main.cpp
    ${SOURCE_PATHS}
)

add_library(${LIB_NAME}
    ${SOURCE_PATHS}
)

target_include_directories(${PROJECT_NAME}
    PUBLIC
    ${INCLUDE_DIRS}
)

add_subdirectory(external/ClParser)

target_link_libraries(${LIB_NAME} ClParser Qt${QT_VERSION_MAJOR}::Core)
target_link_libraries(${PROJECT_NAME} ClParser Qt${QT_VERSION_MAJOR}::Core)


