add_executable(fw_unit test_init.c)
target_link_libraries(fw_unit PRIVATE fw_core)
target_compile_options(fw_unit PRIVATE -Wall -Wextra -Wpedantic)

add_test(NAME fw_unit COMMAND fw_unit)
