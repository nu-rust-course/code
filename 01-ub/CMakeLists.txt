cmake_minimum_required(VERSION 3.3)
project(ub CXX)

macro (add_program name)
    add_executable(${name} ${ARGN})
    set_target_properties(${name} PROPERTIES
            CXX_STANDARD            17
            CXX_STANDARD_REQUIRED   On
            CXX_EXTENSIONS          Off)
endmacro (add_program)

add_program(div_mul cxx-src/div_mul.cpp)
add_program(int_max cxx-src/int_max.cxx)
add_program(uninitialized cxx-src/uninitialized.cxx)
add_program(array cxx-src/array.cxx)
add_program(iterator cxx-src/iterator.cxx)
