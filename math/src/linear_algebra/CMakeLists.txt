cmake_minimum_required(VERSION 3.28)
project(linear_algebra C)

set(CMAKE_C_STANDARD 11)

add_executable(linear_algebra library.c
        matrix.c
        string.c
        string.h
        matrix.h)
#add_library(linear_algebra SHARED library.c
#        matrix.c)
