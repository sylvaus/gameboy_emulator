cmake_minimum_required(VERSION 3.10)
project(emulator)

option(TESTS "Build test programs" OFF)
option(TEST_COVERAGE "Generate test coverage" OFF)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic")

if(NOT EXISTS ${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
    message(WARNING "The file conanbuildinfo.cmake doesn't exist, running conan install to create it ${PROJECT_BINARY_DIR}")
    execute_process(
        COMMAND echo %PATH
        WORKING_DIRECTORY  ${PROJECT_BINARY_DIR}
        OUTPUT_FILE ${PROJECT_BINARY_DIR}/conan_install.log
        ERROR_FILE  ${PROJECT_BINARY_DIR}/conan_install.error.log
    )
endif()

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)

include_directories("include")

add_executable(
        emulator main.cpp src/cartridge.cpp src/logging.cpp
        src/memory_map.cpp src/romram_controller.cpp src/generated/instructions.cpp
        src/registers.cpp
)
if (TESTS)
    enable_testing()
    if (TEST_COVERAGE)
        set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} --coverage")
    endif()

    file(GLOB_RECURSE TESTS_SRC ${PROJECT_SOURCE_DIR}/tests/test_*.cpp)
    add_executable(tests ${TESTS_SRC} src/registers.cpp src/generated/instructions.cpp)
    target_link_libraries(tests ${CONAN_LIBS})
    add_test(all_tests tests)
endif()