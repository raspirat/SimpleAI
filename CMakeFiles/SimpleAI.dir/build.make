# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.28

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /opt/cmake-3.28.2-linux-x86_64/bin/cmake

# The command to remove a file.
RM = /opt/cmake-3.28.2-linux-x86_64/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/malte/code/SimpleAI

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/malte/code/SimpleAI

# Include any dependencies generated for this target.
include CMakeFiles/SimpleAI.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/SimpleAI.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/SimpleAI.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/SimpleAI.dir/flags.make

SimpleAI_autogen/timestamp: /usr/lib/qt6/libexec/moc
SimpleAI_autogen/timestamp: CMakeFiles/SimpleAI.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --blue --bold --progress-dir=/home/malte/code/SimpleAI/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Automatic MOC and UIC for target SimpleAI"
	/opt/cmake-3.28.2-linux-x86_64/bin/cmake -E cmake_autogen /home/malte/code/SimpleAI/CMakeFiles/SimpleAI_autogen.dir/AutogenInfo.json ""
	/opt/cmake-3.28.2-linux-x86_64/bin/cmake -E touch /home/malte/code/SimpleAI/SimpleAI_autogen/timestamp

SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: qml.qrc
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: CMakeFiles/SimpleAI_autogen.dir/AutoRcc_qml_EWIEGA46WW_Info.json
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: NewDataset.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: Font.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: MainMenu.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: NewProject.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: main.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: NewProfile.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: NewModel.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: components/Dropdown.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: components/CustomTextField.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: components/Style.qml
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-project-light.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-profile-light.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-dataset.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/type.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-model-light.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/browse.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/browse-light.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/logo.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-project.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-model.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/plain.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-profile.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/new-dataset-light.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/home.png
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/SVG/logo.svg
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/SVG/new-dataset.svg
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/SVG/new-model.svg
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/SVG/new-profile.svg
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/SVG/new-project.svg
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: assets/SVG/plain.svg
SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp: /usr/lib/qt6/libexec/rcc
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --blue --bold --progress-dir=/home/malte/code/SimpleAI/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Automatic RCC for qml.qrc"
	/opt/cmake-3.28.2-linux-x86_64/bin/cmake -E cmake_autorcc /home/malte/code/SimpleAI/CMakeFiles/SimpleAI_autogen.dir/AutoRcc_qml_EWIEGA46WW_Info.json 

CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o: CMakeFiles/SimpleAI.dir/flags.make
CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o: SimpleAI_autogen/mocs_compilation.cpp
CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o: CMakeFiles/SimpleAI.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/malte/code/SimpleAI/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o -MF CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o.d -o CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o -c /home/malte/code/SimpleAI/SimpleAI_autogen/mocs_compilation.cpp

CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/malte/code/SimpleAI/SimpleAI_autogen/mocs_compilation.cpp > CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.i

CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/malte/code/SimpleAI/SimpleAI_autogen/mocs_compilation.cpp -o CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.s

CMakeFiles/SimpleAI.dir/main.cpp.o: CMakeFiles/SimpleAI.dir/flags.make
CMakeFiles/SimpleAI.dir/main.cpp.o: main.cpp
CMakeFiles/SimpleAI.dir/main.cpp.o: CMakeFiles/SimpleAI.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/malte/code/SimpleAI/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/SimpleAI.dir/main.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/SimpleAI.dir/main.cpp.o -MF CMakeFiles/SimpleAI.dir/main.cpp.o.d -o CMakeFiles/SimpleAI.dir/main.cpp.o -c /home/malte/code/SimpleAI/main.cpp

CMakeFiles/SimpleAI.dir/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/SimpleAI.dir/main.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/malte/code/SimpleAI/main.cpp > CMakeFiles/SimpleAI.dir/main.cpp.i

CMakeFiles/SimpleAI.dir/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/SimpleAI.dir/main.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/malte/code/SimpleAI/main.cpp -o CMakeFiles/SimpleAI.dir/main.cpp.s

CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o: CMakeFiles/SimpleAI.dir/flags.make
CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o: SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp
CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o: CMakeFiles/SimpleAI.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/malte/code/SimpleAI/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building CXX object CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o -MF CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o.d -o CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o -c /home/malte/code/SimpleAI/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp

CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/malte/code/SimpleAI/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp > CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.i

CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/malte/code/SimpleAI/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp -o CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.s

# Object files for target SimpleAI
SimpleAI_OBJECTS = \
"CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o" \
"CMakeFiles/SimpleAI.dir/main.cpp.o" \
"CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o"

# External object files for target SimpleAI
SimpleAI_EXTERNAL_OBJECTS =

SimpleAI: CMakeFiles/SimpleAI.dir/SimpleAI_autogen/mocs_compilation.cpp.o
SimpleAI: CMakeFiles/SimpleAI.dir/main.cpp.o
SimpleAI: CMakeFiles/SimpleAI.dir/SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp.o
SimpleAI: CMakeFiles/SimpleAI.dir/build.make
SimpleAI: /usr/lib/x86_64-linux-gnu/libQt6Quick.so.6.2.4
SimpleAI: /usr/lib/x86_64-linux-gnu/libQt6QmlModels.so.6.2.4
SimpleAI: /usr/lib/x86_64-linux-gnu/libQt6Qml.so.6.2.4
SimpleAI: /usr/lib/x86_64-linux-gnu/libQt6Network.so.6.2.4
SimpleAI: /usr/lib/x86_64-linux-gnu/libQt6OpenGL.so.6.2.4
SimpleAI: /usr/lib/x86_64-linux-gnu/libQt6Gui.so.6.2.4
SimpleAI: /usr/lib/x86_64-linux-gnu/libQt6Core.so.6.2.4
SimpleAI: /usr/lib/x86_64-linux-gnu/libGLX.so
SimpleAI: /usr/lib/x86_64-linux-gnu/libOpenGL.so
SimpleAI: CMakeFiles/SimpleAI.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/malte/code/SimpleAI/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Linking CXX executable SimpleAI"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/SimpleAI.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/SimpleAI.dir/build: SimpleAI
.PHONY : CMakeFiles/SimpleAI.dir/build

CMakeFiles/SimpleAI.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/SimpleAI.dir/cmake_clean.cmake
.PHONY : CMakeFiles/SimpleAI.dir/clean

CMakeFiles/SimpleAI.dir/depend: SimpleAI_autogen/EWIEGA46WW/qrc_qml.cpp
CMakeFiles/SimpleAI.dir/depend: SimpleAI_autogen/timestamp
	cd /home/malte/code/SimpleAI && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/malte/code/SimpleAI /home/malte/code/SimpleAI /home/malte/code/SimpleAI /home/malte/code/SimpleAI /home/malte/code/SimpleAI/CMakeFiles/SimpleAI.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/SimpleAI.dir/depend

