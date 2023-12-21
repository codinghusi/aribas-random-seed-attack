cmake_minimum_required(VERSION 3.0)
set(CMAKE_C_COMPILER "gcc")

project(CRand C)
add_library(crand STATIC crand.c)
install(TARGETS crand DESTINATION .)