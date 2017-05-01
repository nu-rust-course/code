cmake_minimum_required(VERSION 3.3)
project(wf_srsw_fifo CXX)
include(.cmake/CMakeLists.txt)

add_test_program(fifo src/fifo.cpp)
