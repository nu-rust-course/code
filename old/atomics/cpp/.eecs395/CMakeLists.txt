# Turn on CTest
enable_testing()

# Make EECS230 and student headers visible
include_directories(.eecs395/include)
include_directories(src)

# UnitTest++
set(UNITTESTCPP_ROOT .eecs395/lib/unittest-cpp)
add_subdirectory(${UNITTESTCPP_ROOT})
include_directories(${UNITTESTCPP_ROOT})

# Adds a program with the given name and source files, and sets the
# language to C++ 14
macro (add_program name)
    add_executable(${name} ${ARGN})
    set_property(TARGET ${name} PROPERTY CXX_STANDARD 14)
    set_property(TARGET ${name} PROPERTY CXX_STANDARD_REQUIRED On)
endmacro (add_program)

# Adds a test program with the given name and source files
macro (add_test_program name)
    add_program(${name} .eecs395/lib/unittest-main.cpp ${ARGN})
    target_link_libraries(${name} UnitTest++)
    add_test(Test_${name} ${name})
endmacro (add_test_program)

