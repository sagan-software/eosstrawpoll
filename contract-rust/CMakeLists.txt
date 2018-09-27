cmake_minimum_required(VERSION 3.5)
project(eosio_contracts VERSION 1.2.0)

set(EOSIO_DEPENDENCY "1.1")
set(EOSIO_WASMSDK_DEPENDENCY "1.1")

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
   set(TEST_BUILD_TYPE "Debug")
   set(CMAKE_BUILD_TYPE "Release")
else()
   set(TEST_BUILD_TYPE ${CMAKE_BUILD_TYPE})
endif()

if(EOSIO_ROOT STREQUAL "" OR NOT EOSIO_ROOT)
   set(EOSIO_ROOT "/usr/local/eosio")
endif()

if(EOSIO_WASMSDK_ROOT STREQUAL "" OR NOT EOSIO_WASMSDK_ROOT)
   set(EOSIO_WASMSDK_ROOT "/usr/local/eosio.wasmsdk")
endif()

list(APPEND CMAKE_MODULE_PATH ${EOSIO_WASMSDK_ROOT}/lib/cmake)
include(EosioWasmToolchain)

### Check the version of wasmsdk
string(FIND "${EOSIO_WASMSDK_VERSION}" "${EOSIO_WASMSDK_DEPENDENCY}" output)

if (NOT "${output}" EQUAL 0)
   message(FATAL_ERROR "Incorrect EOSIO.WasmSDK version, please use version ${EOSIO_WASMSDK_DEPENDENCY}.x")
endif()

include_directories(AFTER ${BOOST_ROOT}/include)


add_executable(hello.wasm ${CMAKE_CURRENT_SOURCE_DIR}/reference/hello.cpp)
target_include_directories(hello.wasm
   PUBLIC
   ${CMAKE_CURRENT_SOURCE_DIR}/include)

set_target_properties(hello.wasm
   PROPERTIES
   RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")

configure_file("${CMAKE_CURRENT_SOURCE_DIR}/reference/hello.abi" "${CMAKE_CURRENT_BINARY_DIR}" COPYONLY)



add_executable(tic_tac_toe.wasm ${CMAKE_CURRENT_SOURCE_DIR}/reference/tic_tac_toe.cpp)
target_include_directories(tic_tac_toe.wasm
   PUBLIC
   ${CMAKE_CURRENT_SOURCE_DIR}/include)

set_target_properties(tic_tac_toe.wasm
   PROPERTIES
   RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")

configure_file("${CMAKE_CURRENT_SOURCE_DIR}/reference/tic_tac_toe.abi" "${CMAKE_CURRENT_BINARY_DIR}" COPYONLY)



add_executable(eosio.token.wasm ${CMAKE_CURRENT_SOURCE_DIR}/reference/eosio.token.cpp)
target_include_directories(eosio.token.wasm
   PUBLIC
   ${CMAKE_CURRENT_SOURCE_DIR}/include)

set_target_properties(eosio.token.wasm
   PROPERTIES
   RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")

configure_file("${CMAKE_CURRENT_SOURCE_DIR}/reference/eosio.token.abi" "${CMAKE_CURRENT_BINARY_DIR}" COPYONLY)


if (APPLE)
   set(OPENSSL_ROOT "/usr/local/opt/openssl")
elseif (UNIX)
   set(OPENSSL_ROOT "/usr/include/openssl")
endif()
set(SECP256K1_ROOT "/usr/local")
