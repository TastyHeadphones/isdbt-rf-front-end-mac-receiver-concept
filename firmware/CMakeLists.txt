cmake_minimum_required(VERSION 3.20)
project(isdbt_receiver_firmware C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

option(BUILD_TESTS "Build firmware tests" ON)

add_library(fw_core
  src/bsp.c
  src/bridge.c
  src/control.c
  src/demod.c
  src/log.c
  src/self_test.c
  src/tuner.c
  drivers/bridge_driver_stub.c
  drivers/demod_driver_stub.c
  drivers/tuner_driver_stub.c
)

target_include_directories(fw_core
  PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_compile_options(fw_core PRIVATE -Wall -Wextra -Wpedantic)

add_executable(fw_demo src/main.c)
target_link_libraries(fw_demo PRIVATE fw_core)

if(BUILD_TESTS)
  enable_testing()
  add_subdirectory(tests)
endif()
