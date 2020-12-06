cmake_minimum_required(VERSION 3.10)
project(emulator)

set(CMAKE_CXX_STANDARD 17)

include_directories("include")

add_executable(emulator main.cpp src/cartridge.cpp src/logging.cpp src/memory_map.cpp src/memory_controller.cpp)

if (TESTS)
    enable_testing()
    find_package(GTest REQUIRED)
    find_package(Threads REQUIRED)
    include_directories(${GTEST_INCLUDE_DIRS})

    file(GLOB TESTS_SRC ${PROJECT_SOURCE_DIR}/tests/*.cpp)
    add_executable(tests ${TESTS_SRC})
    target_link_libraries(tests ${GTEST_BOTH_LIBRARIES} ${CMAKE_THREAD_LIBS_INIT})
    add_test(all_tests tests)
endif()