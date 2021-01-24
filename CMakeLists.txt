cmake_minimum_required(VERSION 3.10)
project(emulator)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic")

include_directories("include")

add_executable(
        emulator main.cpp src/cartridge.cpp src/logging.cpp
        src/memory_map.cpp src/romram_controller.cpp src/generated/instructions.cpp
        src/registers.cpp
)
if (TESTS)
    enable_testing()
    find_package(GTest REQUIRED)
    find_package(Threads REQUIRED)
    include_directories(${GTEST_INCLUDE_DIRS})

    file(GLOB_RECURSE TESTS_SRC ${PROJECT_SOURCE_DIR}/tests/test_*.cpp)
    add_executable(tests ${TESTS_SRC} src/registers.cpp src/generated/instructions.cpp)
    target_link_libraries(tests ${GTEST_BOTH_LIBRARIES} gmock ${CMAKE_THREAD_LIBS_INIT})
    add_test(all_tests tests)
endif()