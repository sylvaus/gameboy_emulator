cmake_minimum_required(VERSION 3.20)
project(emulator)

option(TESTS "Build test programs" OFF)
option(TEST_COVERAGE "Generate test coverage" OFF)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic")

# Conan management
if(NOT EXISTS ${CMAKE_BINARY_DIR}/conan_paths.cmake)
    message(WARNING "The file conanbuildinfo.cmake doesn't exist, running conan install to create it ${PROJECT_BINARY_DIR}")
    execute_process(
        COMMAND conan install ..
        WORKING_DIRECTORY  ${PROJECT_BINARY_DIR}
        OUTPUT_FILE ${PROJECT_BINARY_DIR}/conan_install.log
        ERROR_FILE  ${PROJECT_BINARY_DIR}/conan_install.error.log
    )
endif()
include(${CMAKE_BINARY_DIR}/conan_paths.cmake)

include_directories("include")

add_library(
        emulator_lib
        src/cartridge.cpp
        src/logging.cpp
        src/io/joypad.cpp
        src/io/serial.cpp
        src/memory/memory_map.cpp
        src/memory/memory.cpp
        src/memory/mbc.cpp
        src/memory/registers.cpp
        src/memory/ram.cpp
        src/time/timer.cpp
        src/sound/controller.cpp
        src/video/controller.cpp
        src/generated/instructions.cpp
)

add_executable(emulator main.cpp)
target_link_libraries(emulator emulator_lib)

if (TESTS)
    # Google test management: https://google.github.io/googletest/quickstart-cmake.html
    include(FetchContent)
    FetchContent_Declare(
            googletest
            URL https://github.com/google/googletest/archive/e2239ee6043f73722e7aa812a459f54a28552929.zip
    )
    # For Windows: Prevent overriding the parent project's compiler/linker settings
    set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(googletest)

    enable_testing()
    if (TEST_COVERAGE)
        set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} --coverage")
    endif()

    file(GLOB_RECURSE TESTS_SRC ${PROJECT_SOURCE_DIR}/tests/test_*.cpp)
    add_executable(
            tests ${TESTS_SRC}
            tests/instructions/fixtures/instruction.cpp
    )
    target_link_libraries(tests gtest_main gmock emulator_lib ${CMAKE_THREAD_LIBS_INIT})

    include(GoogleTest)
    gtest_discover_tests(tests)
endif()