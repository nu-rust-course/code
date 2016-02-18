# Turn on CTest
enable_testing()

# Make EECS230 and student headers visible
include_directories(.eecs230/include)
include_directories(src)

# UnitTest++
set(UNITTESTCPP_ROOT .eecs230/lib/unittest-cpp)
add_subdirectory(${UNITTESTCPP_ROOT})
include_directories(${UNITTESTCPP_ROOT})

# Adds a program with the given name and source files, and sets the
# language to C++ 11
macro (add_program name)
    add_executable(${name} ${ARGN})
    set_property(TARGET ${name} PROPERTY CXX_STANDARD 11)
    set_property(TARGET ${name} PROPERTY CXX_STANDARD_REQUIRED On)
endmacro (add_program)

# Adds a test program with the given name and source files
macro (add_test_program name)
    add_program(${name} .eecs230/lib/unittest-main.cpp ${ARGN})
    target_link_libraries(${name} UnitTest++)
    add_test(Test_${name} ${name})
endmacro (add_test_program)

# Copy source files into build directory so that evaluation.cpp can see
# them
file(GLOB SRC_FILES "src/*")
file(COPY ${SRC_FILES} DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/src/)

###
### Setting some flags, portably
###

# Sets the given flag IFF the compiler supports it
include(CheckCXXCompilerFlag)
macro (safe_add_cxx_flag flag)
    check_cxx_compiler_flag(-${flag} HAS_${flag})
    if (HAS_${flag})
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -${flag}")
    endif()
endmacro (safe_add_cxx_flag)

safe_add_cxx_flag(O0)           # Don't optimize
safe_add_cxx_flag(g)            # Do include debug symbols

