cmake_minimum_required(VERSION 3.27)
project(librazer C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/out)
set(CMAKE_PREFIX_PATH ${CMAKE_SOURCE_DIR}/lib)

find_path(LIBUSB_INCLUDE_DIRS NAMES libusb/libusb.h PATHS ${CMAKE_SOURCE_DIR}/include) #libusb-1.0 header files
find_library(LIBUSB_LIBRARIES NAMES libusb-1.0) #Static libusb-1.0 library

message(STATUS "LIBUSB_INCLUDE_DIRS: ${LIBUSB_INCLUDE_DIRS}")
message(STATUS "LIBUSB_LIBRARIES: ${LIBUSB_LIBRARIES}")

add_library(razer SHARED src/rzcommon.c
        src/rzcommon.h
        src/rzdevices.h
        src/usbcommon.c
        src/usbcommon.h
        src/chromacommon.c
        src/chromacommon.h
)

target_include_directories(razer PRIVATE ${LIBUSB_INCLUDE_DIRS}) #libusb-1.0 header files
target_link_libraries(razer PRIVATE ${LIBUSB_LIBRARIES}) #Static libusb-1.0 library

add_executable(librazer_test src/test.c)
target_include_directories(librazer_test PRIVATE ${LIBUSB_INCLUDE_DIRS}) #libusb-1.0 header files
target_link_libraries(librazer_test PRIVATE razer ${LIBUSB_LIBRARIES})

if (WIN32)
    #Copy DLLs if changed
    add_custom_command(TARGET librazer_test POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_directory_if_different
        "${PROJECT_SOURCE_DIR}/bin"
        ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
endif (WIN32)