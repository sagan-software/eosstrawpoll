#!/bin/bash

SRC_DIR="/contracts"
EOS_BUILD_DIR="/eos/build"
EOS_SRC_DIR="/eos"
CORE_SYMBOL="SYS"

contract_names=`find $SRC_DIR -mindepth 1 -maxdepth 1 -type d -exec basename {} \;`

build_contract_cmakelists () {
    read -r -d '' OUTPUT << END
file(GLOB ABI_FILES "*.abi")
configure_file("\${ABI_FILES}" "\${CMAKE_CURRENT_BINARY_DIR}" COPYONLY)

add_wast_executable(TARGET $1
  INCLUDE_FOLDERS "\${STANDARD_INCLUDE_FOLDERS}"
  LIBRARIES libc libc++ eosiolib
  DESTINATION_FOLDER \${CMAKE_CURRENT_BINARY_DIR}
)
END
    echo "$OUTPUT"
}

build_add_subdirectories () {
    for contract_name in $contract_names
    do
        echo "add_subdirectory($contract_name)"
    done
}

build_contracts_cmakelists () {
    subdirectories=`build_add_subdirectories`
    read -r -d '' OUTPUT << END
# will be implictly used for any compilation unit if not overrided by SYSTEM_INCLUDE_FOLDERS parameter
# these directories go as -isystem <dir> to avoid warnings from code of third-party libraries
set(DEFAULT_SYSTEM_INCLUDE_FOLDERS \${CMAKE_SOURCE_DIR}/contracts/libc++/upstream/include \${CMAKE_SOURCE_DIR}/contracts/musl/upstream/include \${Boost_INCLUDE_DIR})

set(STANDARD_INCLUDE_FOLDERS \${CMAKE_SOURCE_DIR}/contracts \${CMAKE_SOURCE_DIR}/externals/magic_get/include)

add_subdirectory(eosiolib)
add_subdirectory(musl)
add_subdirectory(libc++)
add_subdirectory(simple.token)
add_subdirectory(eosio.token)
add_subdirectory(eosio.msig)
add_subdirectory(multi_index_test)
add_subdirectory(eosio.system)
add_subdirectory(identity)
add_subdirectory(stltest)
add_subdirectory(exchange)
add_subdirectory(test.inline)

#add_subdirectory(bancor)
add_subdirectory(hello)
add_subdirectory(asserter)
add_subdirectory(infinite)
add_subdirectory(proxy)
add_subdirectory(test_api)
add_subdirectory(test_api_mem)
add_subdirectory(test_api_db)
add_subdirectory(test_api_multi_index)
add_subdirectory(test_ram_limit)
#add_subdirectory(social)
add_subdirectory(eosio.bios)
add_subdirectory(noop)
add_subdirectory(dice)
add_subdirectory(tic_tac_toe)
add_subdirectory(payloadless)
$subdirectories


file(GLOB SKELETONS RELATIVE \${CMAKE_SOURCE_DIR}/contracts "skeleton/*")

# Documented multiple output support is broken, so properly setting up the multiple
# dependencies in the custom target is not possible.  (CMake 3.5)
add_custom_command(OUTPUT share/eosio/skeleton/skeleton.cpp
                   COMMAND \${CMAKE_COMMAND} -E make_directory ../share/eosio/skeleton
                   COMMAND \${CMAKE_COMMAND} -E copy_directory \${CMAKE_CURRENT_SOURCE_DIR}/skeleton ../share/eosio/skeleton
                   DEPENDS \${SKELETONS}
                   COMMENT Copying skeleton contract...
                   VERBATIM)
add_custom_target(copy_skeleton_contract ALL DEPENDS share/eosio/skeleton/skeleton.cpp)

install(DIRECTORY eosiolib DESTINATION \${CMAKE_INSTALL_FULL_INCLUDEDIR})
install(DIRECTORY eosio.system DESTINATION \${CMAKE_INSTALL_FULL_INCLUDEDIR})
install(DIRECTORY musl DESTINATION \${CMAKE_INSTALL_FULL_INCLUDEDIR})
install(DIRECTORY libc++ DESTINATION \${CMAKE_INSTALL_FULL_INCLUDEDIR})
install(DIRECTORY skeleton DESTINATION \${CMAKE_INSTALL_FULL_DATAROOTDIR}/eosio)
install_directory_permissions(DIRECTORY \${CMAKE_INSTALL_FULL_DATAROOTDIR}/eosio)
END
    echo "$OUTPUT"
}


build_unittests_cmakelists () {
    read -r -d '' OUTPUT << END
#file(GLOB COMMON_SOURCES "common/*.cpp")

find_package( Gperftools QUIET )
if( GPERFTOOLS_FOUND )
    message( STATUS "Found gperftools; compiling tests with TCMalloc")
    list( APPEND PLATFORM_SPECIFIC_LIBS tcmalloc )
endif()

find_package(LLVM 4.0 REQUIRED CONFIG)

link_directories(\${LLVM_LIBRARY_DIR})

set( CMAKE_CXX_STANDARD 14 )

add_subdirectory(contracts)

configure_file(\${CMAKE_CURRENT_SOURCE_DIR}/include/config.hpp.in \${CMAKE_CURRENT_BINARY_DIR}/include/config.hpp ESCAPE_QUOTES)

file(GLOB UNIT_TESTS "*.cpp")

add_executable( unit_test \${UNIT_TESTS} \${WASM_UNIT_TESTS} )
target_link_libraries( unit_test eosio_chain chainbase eosio_testing eos_utilities abi_generator fc \${PLATFORM_SPECIFIC_LIBS} )

target_include_directories( unit_test PUBLIC
                            \${CMAKE_SOURCE_DIR}/libraries/testing/include
                            \${CMAKE_SOURCE_DIR}/contracts
                            \${CMAKE_BINARY_DIR}/contracts
                            \${CMAKE_CURRENT_SOURCE_DIR}/contracts
                            \${CMAKE_CURRENT_BINARY_DIR}/contracts
                            \${CMAKE_CURRENT_BINARY_DIR}/include )
add_dependencies(unit_test asserter test_api test_api_mem test_api_db test_ram_limit test_api_multi_index exchange eosio.token proxy identity identity_test stltest infinite eosio.system eosio.token eosio.bios test.inline multi_index_test noop dice eosio.msig payloadless tic_tac_toe deferred_test $contract_names)

#Manually run unit_test for all supported runtimes
#To run unit_test with all log from blockchain displayed, put --verbose after --, i.e. unit_test -- --verbose
add_test(NAME unit_test_binaryen COMMAND unit_test
 -t \!wasm_tests/weighted_cpu_limit_tests
 --report_level=detailed --color_output -- --binaryen)
add_test(NAME unit_test_wavm COMMAND unit_test
 -t \!wasm_tests/weighted_cpu_limit_tests
 --report_level=detailed --color_output --catch_system_errors=no -- --wavm)

if(ENABLE_COVERAGE_TESTING)

  set(Coverage_NAME \${PROJECT_NAME}_ut_coverage)

  if(NOT LCOV_PATH)
    message(FATAL_ERROR "lcov not found! Aborting...")
  endif() # NOT LCOV_PATH

  if(NOT LLVMCOV_PATH)
    message(FATAL_ERROR "llvm-cov not found! Aborting...")
  endif() # NOT LCOV_PATH

  if(NOT GENHTML_PATH)
    message(FATAL_ERROR "genhtml not found! Aborting...")
  endif() # NOT GENHTML_PATH

  # no spaces allowed within tests list
  set(ctest_tests 'unit_test_binaryen|unit_test_wavm')
  set(ctest_exclude_tests '')

  # Setup target
  add_custom_target(\${Coverage_NAME}

    # Cleanup lcov
    COMMAND \${LCOV_PATH} --directory . --zerocounters

    # Run tests
    COMMAND ./tools/ctestwrapper.sh -R \${ctest_tests} -E \${ctest_exclude_tests}

    COMMAND \${LCOV_PATH} --directory . --capture --gcov-tool ./tools/llvm-gcov.sh --output-file \${Coverage_NAME}.info

    COMMAND \${LCOV_PATH} -remove \${Coverage_NAME}.info '*/boost/*' '/usr/lib/*' '/usr/include/*' '*/externals/*' '*/fc/*' '*/wasm-jit/*' --output-file \${Coverage_NAME}_filtered.info

    COMMAND \${GENHTML_PATH} -o \${Coverage_NAME} \${PROJECT_BINARY_DIR}/\${Coverage_NAME}_filtered.info

    COMMAND if [ "\$CI" != "true" ]\; then \${CMAKE_COMMAND} -E remove \${Coverage_NAME}.base \${Coverage_NAME}.info \${Coverage_NAME}_filtered.info \${Coverage_NAME}.total \${PROJECT_BINARY_DIR}/\${Coverage_NAME}.info.cleaned \${PROJECT_BINARY_DIR}/\${Coverage_NAME}_filtered.info.cleaned\; fi

    WORKING_DIRECTORY \${PROJECT_BINARY_DIR}
    COMMENT "Resetting code coverage counters to zero. Processing code coverage counters and generating report. Report published in ./\${Coverage_NAME}"
    )

  # Show info where to find the report
  add_custom_command(TARGET \${Coverage_NAME} POST_BUILD
    COMMAND ;
    COMMENT "Open ./\${Coverage_NAME}/index.html in your browser to view the coverage report."
    )
endif()
END
    echo "$OUTPUT"
}

for contract_name in $contract_names; do
    echo
    echo "Setting up files for contract: $contract_name"
    eos_src_dir=$EOS_SRC_DIR/contracts/$contract_name
    eos_build_dir=$EOS_BUILD_DIR/contracts/$contract_name

    #rm -Rf $eos_src_dir $eos_build_dir
    mkdir -p $eos_src_dir $eos_build_dir
    cp -Rf $SRC_DIR/$contract_name/* $eos_src_dir
    cp -Rf $SRC_DIR/$contract_name/* $eos_build_dir
    build_contract_cmakelists $contract_name > $eos_src_dir/CMakeLists.txt
    cp -f $eos_src_dir/CMakeLists.txt $eos_build_dir/CMakeLists.txt

    #eosiocpp --genabi $SRC_DIR/$contract_name/$contract_name.abi $SRC_DIR/$contract_name/$contract_name.cpp
    eosiocpp --outname $SRC_DIR/$contract_name/$contract_name.wast $SRC_DIR/$contract_name/$contract_name.cpp

    cp $SRC_DIR/$contract_name/$contract_name.* $eos_src_dir
    cp $SRC_DIR/$contract_name/$contract_name.* $eos_build_dir
done

# build_contracts_cmakelists > $EOS_SRC_DIR/contracts/CMakeLists.txt
cp $EOS_SRC_DIR/contracts/CMakeLists.txt $EOS_BUILD_DIR/contracts

# test_files=`find $SRC_DIR -name '*_tests.cpp' -type f`
# for test_file in $test_files
# do
#     cp -Rf $test_file $EOS_SRC_DIR/unittests
#     cp -Rf $test_file $EOS_BUILD_DIR/unittests
# done

# build_unittests_cmakelists > $EOS_SRC_DIR/unittests/CMakeLists.txt
# cp $EOS_SRC_DIR/unittests/CMakeLists.txt $EOS_BUILD_DIR/unittests

# Build

cmake \
    -H"$EOS_SRC_DIR" \
    -B"$EOS_BUILD_DIR" \
    -GNinja \
    -DCMAKE_BUILD_TYPE=Release \
    -DWASM_ROOT=/opt/wasm \
    -DCMAKE_CXX_COMPILER=clang++ \
    -DCMAKE_C_COMPILER=clang \
    -DCMAKE_INSTALL_PREFIX=$EOS_BUILD_DIR \
    -DSecp256k1_ROOT_DIR=/usr/local \
    -DBUILD_MONGO_DB_PLUGIN=true \
    -DCORE_SYMBOL_NAME=$CORE_SYMBOL

cmake --build $EOS_BUILD_DIR --target install

# test_names=`grep -oPh '(?<=BOOST_AUTO_TEST_SUITE\()(.+)(?=\))' $SRC_DIR/**/*.cpp`

# for test_name in $test_names; do
#     echo
#     echo "Running tests: $test_name"
#     $EOS_BUILD_DIR/unittests/unit_test --show_progress=yes --run_test=$test_name
# done
