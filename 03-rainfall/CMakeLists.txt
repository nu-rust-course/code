cmake_minimum_required(VERSION 3.3)
project(rainfall CXX)

macro (add_program name)
    add_executable(${name} ${ARGN})
    set_target_properties(${name} PROPERTIES
            CXX_STANDARD            17
            CXX_STANDARD_REQUIRED   On
            CXX_EXTENSIONS          Off)
endmacro (add_program)

add_program(rainfall cxx-src/rainfall.cxx)
