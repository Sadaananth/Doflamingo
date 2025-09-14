add_subdirectory(core)

cmake_minimum_required(VERSION 3.21)
project(my_application LANGUAGES CXX)

# Note: Use find_package(Slint) instead of the following three commands,
# if you prefer the package approach.
include(FetchContent)
FetchContent_Declare(
    Slint
    GIT_REPOSITORY https://github.com/slint-ui/slint.git
    # `release/1` will auto-upgrade to the latest Slint >= 1.0.0 and < 2.0.0
    # `release/1.0` will auto-upgrade to the latest Slint >= 1.0.0 and < 1.1.0
    GIT_TAG release/1
    SOURCE_SUBDIR api/cpp
)
FetchContent_MakeAvailable(Slint)

add_executable(Doflamingo main.cpp)
slint_target_sources(my_application my_application_ui.slint)
target_link_libraries(my_application PRIVATE Slint::Slint)

