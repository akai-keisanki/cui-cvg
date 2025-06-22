cmake_minimum_required(VERSION 3.28.3)
project(CUICVG LANGUAGES CXX)

add_executable(cli main.cc)

target_include_directories(cli PRIVATE include)

target_compile_options(cli PRIVATE -std=c++20)
target_compile_features(cli PRIVATE cxx_std_20)
