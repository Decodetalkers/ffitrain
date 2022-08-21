cmake_minimum_required(VERSION 3.10)
project("fromrust")

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 17)
include_directories("include")
add_executable(${PROJECT_NAME}
  cpp/main.cpp
)
target_link_libraries(
  ${PROJECT_NAME} PRIVATE
  ${CMAKE_CURRENT_LIST_DIR}/../target/release/libdynamiclink.so
)
