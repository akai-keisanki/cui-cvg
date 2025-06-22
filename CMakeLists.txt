cmake_minimum_required(VERSION 3.28.3)
project(CUICVG LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(cvg STATIC lib/cvg.cc)
target_include_directories(cvg PRIVATE include)

add_executable(cli src/main.cc)
target_include_directories(cli PRIVATE include)
target_link_libraries(cli PRIVATE cvg)

target_compile_options(cli PRIVATE -std=c++20)
target_compile_features(cli PRIVATE cxx_std_20)
