# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.10

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /contract

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /build

# Include any dependencies generated for this target.
include CMakeFiles/eosio.token.wasm.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/eosio.token.wasm.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/eosio.token.wasm.dir/flags.make

CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o: CMakeFiles/eosio.token.wasm.dir/flags.make
CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o: /contract/reference/eosio.token.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o"
	/usr/local/eosio.wasmsdk//bin/eosio-cpp  $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o -c /contract/reference/eosio.token.cpp

CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.i"
	/usr/local/eosio.wasmsdk//bin/eosio-cpp $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /contract/reference/eosio.token.cpp > CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.i

CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.s"
	/usr/local/eosio.wasmsdk//bin/eosio-cpp $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /contract/reference/eosio.token.cpp -o CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.s

CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.requires:

.PHONY : CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.requires

CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.provides: CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.requires
	$(MAKE) -f CMakeFiles/eosio.token.wasm.dir/build.make CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.provides.build
.PHONY : CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.provides

CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.provides.build: CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o


# Object files for target eosio.token.wasm
eosio_token_wasm_OBJECTS = \
"CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o"

# External object files for target eosio.token.wasm
eosio_token_wasm_EXTERNAL_OBJECTS =

eosio.token.wasm: CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o
eosio.token.wasm: CMakeFiles/eosio.token.wasm.dir/build.make
eosio.token.wasm: CMakeFiles/eosio.token.wasm.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX executable eosio.token.wasm"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/eosio.token.wasm.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/eosio.token.wasm.dir/build: eosio.token.wasm

.PHONY : CMakeFiles/eosio.token.wasm.dir/build

CMakeFiles/eosio.token.wasm.dir/requires: CMakeFiles/eosio.token.wasm.dir/reference/eosio.token.cpp.o.requires

.PHONY : CMakeFiles/eosio.token.wasm.dir/requires

CMakeFiles/eosio.token.wasm.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/eosio.token.wasm.dir/cmake_clean.cmake
.PHONY : CMakeFiles/eosio.token.wasm.dir/clean

CMakeFiles/eosio.token.wasm.dir/depend:
	cd /build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /contract /contract /build /build /build/CMakeFiles/eosio.token.wasm.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/eosio.token.wasm.dir/depend
