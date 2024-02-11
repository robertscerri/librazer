cmake_minimum_required(VERSION 3.28)
project(librazer C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/out)

find_library(HIDAPI_LIBRARIES NAMES hidapi.lib PATHS ${CMAKE_SOURCE_DIR}/lib) #Static hidapi library
find_path(HIDAPI_INCLUDE_DIRS NAMES hidapi/hidapi.h PATHS ${CMAKE_SOURCE_DIR}/include) #hidapi header files

message(STATUS "HIDAPI_INCLUDE_DIRS: ${HIDAPI_INCLUDE_DIRS}")
message(STATUS "HIDAPI_LIBRARIES: ${HIDAPI_LIBRARIES}")

add_executable(test test.c)

target_include_directories(test PRIVATE ${HIDAPI_INCLUDE_DIRS}) #hidapi header files
target_link_libraries(test PRIVATE ${HIDAPI_LIBRARIES}) #Static hidapi library

#Copy DLLs if changed
add_custom_command(TARGET test POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_directory_if_different
        "${PROJECT_SOURCE_DIR}/bin"
        ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})

#add_library(razer SHARED library.c)
