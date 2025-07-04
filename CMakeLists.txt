cmake_minimum_required(VERSION 3.31)
project(cc_validator)

set(CMAKE_CXX_STANDARD 26)

add_executable(cc_validator main.cpp
        validator.cpp
        validator.h)
