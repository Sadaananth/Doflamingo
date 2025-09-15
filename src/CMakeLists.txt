add_subdirectory(core)

find_package(Slint REQUIRED)

set(CMAKE_INSTALL_RPATH "/usr/local/lib;~/tools/Slint-cpp-1.13.1-Linux-x86_64/lib")
set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)

get_target_property(slint_enabled_features Slint::Slint SLINT_ENABLED_FEATURES)
if ("BACKEND_WINIT" IN_LIST slint_enabled_features)
    message("WINIT supported")
endif()

add_executable(Doflamingo main.cpp)
slint_target_sources(Doflamingo ../ui/app-window.slint)
target_link_libraries(Doflamingo PRIVATE Slint::Slint)

