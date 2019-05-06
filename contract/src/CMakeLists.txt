cmake_minimum_required( VERSION 3.5 )

project(contract)

set(EOSIO_WASM_OLD_BEHAVIOR "Off")
find_package(eosio.cdt)

add_contract(contract contract ${CMAKE_CURRENT_SOURCE_DIR}/contract.cpp)

set_target_properties(contract
   PROPERTIES
   RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")
