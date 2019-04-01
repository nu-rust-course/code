cmake_minimum_required(VERSION 3.3)
project(ub CXX)

macro (add_program name)
    add_executable(${name} ${ARGN})
    set_target_properties(${name} PROPERTIES
            CXX_STANDARD            17
            CXX_STANDARD_REQUIRED   On
            CXX_EXTENSIONS          Off)
endmacro (add_program)

add_program(div_mul div_mul.cpp)
add_program(int_max int_max.cpp)
add_program(null_check null_check.cpp)
add_program(uninitialized uninitialized.cpp)
add_program(array array.cpp)
add_program(iterator iterator.cpp)
