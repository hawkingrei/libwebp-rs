cmake_minimum_required(VERSION 3.13)

project(opencv-wrapper)

#set(CMAKE_C_COMPILER "gcc")
#set(CMAKE_CXX_COMPILER "g++")

set(TRUNK_DIR "${CMAKE_CURRENT_SOURCE_DIR}")

option(AVX512 "AVX512 enable" off)
option(SIMD_TEST "Test framework enable" off)
option(CUDA "opencv CUDA enable" off)

if((NOT DEFINED CMAKE_BUILD_TYPE) OR (CMAKE_BUILD_TYPE STREQUAL ""))
    set(CMAKE_BUILD_TYPE "Release")
endif()
message("Build type: '${CMAKE_BUILD_TYPE}'")
if(NOT ${TARGET} STREQUAL "")
    set(CMAKE_SYSTEM_PROCESSOR ${TARGET})
endif()
message("Target: ${CMAKE_SYSTEM_PROCESSOR}")

if(NOT TOOLCHAIN STREQUAL "")
    set(CMAKE_C_COMPILER ${TOOLCHAIN})
    set(CMAKE_CXX_COMPILER ${TOOLCHAIN})
    execute_process(COMMAND ${CMAKE_CXX_COMPILER} -dumpversion OUTPUT_VARIABLE CMAKE_CXX_COMPILER_VERSION)
endif()


set(LIBRARY "STATIC")

message("Library: ${LIBRARY}")
message("Toolchain: ${CMAKE_CXX_COMPILER}")
message("ID: ${CMAKE_CXX_COMPILER_ID}")
message("Version: ${CMAKE_CXX_COMPILER_VERSION}")
message("CMAKE_CXX_FLAGS: ${CMAKE_CXX_FLAGS}")
include_directories(${TRUNK_DIR})

find_package(OpenCV REQUIRED COMPONENTS core imgproc highgui features2d xfeatures2d img_hash imgcodecs objdetect videoio video)
include_directories(${OpenCV_INCLUDE_DIRS})

file(GLOB_RECURSE  OPENCV_WRAPPER 
    ${TRUNK_DIR}/*.cc 
)

file(GLOB_RECURSE  OPENCV_CUDA_WRAPPER ${TRUNK_DIR}/cuda/*.cc)
foreach(v ${OPENCV_CUDA_WRAPPER})
        list(REMOVE_ITEM OPENCV_WRAPPER ${v})
endforeach()

#set(WRAPPER_CXX_FLAGS " -ffunction-sections -fdata-sections -std=c++11 -Wall -Wextra -fPIC -O3 -lstdc++")
set(WRAPPER_CXX_FLAGS " -ffunction-sections -fdata-sections -std=c++11 -Wall -Wextra -fPIC -O0 -g -fno-omit-frame-pointer -lstdc++  -lc++")
set_source_files_properties(${OPENCV_WRAPPER} PROPERTIES COMPILE_FLAGS "${WRAPPER_CXX_FLAGS}")

add_library(opencv-wrapper ${LIBRARY} ${OPENCV_WRAPPER})

target_link_libraries(opencv-wrapper ${OpenCV_LIBS})

install(TARGETS opencv-wrapper
    LIBRARY DESTINATION opencv-wrapper/lib
    ARCHIVE DESTINATION opencv-wrapper/lib
)

