cmake_minimum_required(VERSION 3.3)
project(listset CXX)
include(.eecs230/CMakeLists.txt)

add_test_program(Set_tests test/Set_tests.cpp
        src/Node_base.h
        src/Set_base.h
        src/List_set.h
        src/One_lock_list_set.h
        src/N_lock_list_set.h
        src/HoH_list_set.h
        src/Optimistic_list_set.h
        src/Lazy_list_set.h
        src/Lock_free_list_set.h
        )

add_test_program(Marked_pointer_tests
        test/marked_ptr_tests.cpp
        src/marked_ptr.h)
