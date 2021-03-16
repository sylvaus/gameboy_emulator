cmake_minimum_required(VERSION 3.10)
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

# Google test management
configure_file(tests/googletest-download/CMakeLists.txt.in googletest-download/CMakeLists.txt)
execute_process(COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" .
        RESULT_VARIABLE result
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/googletest-download )
if(result)
    message(FATAL_ERROR "CMake step for googletest failed: ${result}")
endif()
execute_process(COMMAND ${CMAKE_COMMAND} --build .
        RESULT_VARIABLE result
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/googletest-download )
if(result)
    message(FATAL_ERROR "Build step for googletest failed: ${result}")
endif()

# Prevent overriding the parent project's compiler/linker settings on Windows
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

# Add googletest directly to our build. This defines  the gtest and gtest_main targets.
add_subdirectory(
        ${CMAKE_CURRENT_BINARY_DIR}/googletest-src
        ${CMAKE_CURRENT_BINARY_DIR}/googletest-build
        EXCLUDE_FROM_ALL
)


include_directories("include")
add_executable(
        emulator
        main.cpp
        src/cartridge.cpp
        src/logging.cpp
        src/memory/memory_map.cpp
        src/memory/romram_controller.cpp
        src/generated/instructions.cpp
        src/memory/registers.cpp
)

if (TESTS)
    enable_testing()
    if (TEST_COVERAGE)
        set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} --coverage")
    endif()

    file(GLOB_RECURSE TESTS_SRC ${PROJECT_SOURCE_DIR}/tests/test_*.cpp)
    add_executable(
            tests ${TESTS_SRC}
            src/memory/registers.cpp
            src/generated/instructions.cpp
            tests/instructions/fixtures/instruction.cpp
    )
    target_link_libraries(tests gtest_main gmock ${CMAKE_THREAD_LIBS_INIT})
    add_test(all_tests tests)
endif()