# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "flutter_gpu_texture_renderer")
project(${PROJECT_NAME} LANGUAGES CXX)

# This value is used when generating builds using this plugin, so it must
# not be changed.
set(PLUGIN_NAME "flutter_gpu_texture_renderer_plugin")

# Any new source files that you add to the plugin should be added here.
list(APPEND PLUGIN_SOURCES
  "flutter_gpu_texture_renderer_plugin.cc"
)

# Define the plugin library target. Its name must not be changed (see comment
# on PLUGIN_NAME above).
add_library(${PLUGIN_NAME} SHARED
  ${PLUGIN_SOURCES}
)
option(RustBuild "RustBuild" OFF)
set(EPHEMERAL_DIR "" CACHE PATH "Path to the ephemeral directory")

# if(RustBuild)
#   message("ephemeral is ${EPHEMERAL_DIR}")
#   if(NOT EPHEMERAL_DIR)
#     message(FATAL_ERROR "EPHEMERAL_DIR must be set when building with Rust")
#   endif()
#   find_package(PkgConfig REQUIRED)
#   pkg_check_modules(GTK REQUIRED IMPORTED_TARGET gtk+-3.0)
#   pkg_check_modules(GLIB REQUIRED IMPORTED_TARGET glib-2.0)
#   pkg_check_modules(GIO REQUIRED IMPORTED_TARGET gio-2.0)
#   set(FLUTTER_LIBRARY "${EPHEMERAL_DIR}/libflutter_linux_gtk.so")

#   # Published to parent scope for install step.
#   set(FLUTTER_LIBRARY ${FLUTTER_LIBRARY} )
#   set(FLUTTER_ICU_DATA_FILE "${EPHEMERAL_DIR}/icudtl.dat" )
#   set(PROJECT_BUILD_DIR "${PROJECT_DIR}/build/" )
#   set(AOT_LIBRARY "${PROJECT_DIR}/build/lib/libapp.so" )

#   target_link_libraries(flutter INTERFACE "${FLUTTER_LIBRARY}")
#   target_link_libraries(flutter INTERFACE
#     PkgConfig::GTK
#     PkgConfig::GLIB
#     PkgConfig::GIO
#   )
#   add_dependencies(flutter flutter_assemble)

# else()

# endif()


# Apply a standard set of build settings that are configured in the
# application-level CMakeLists.txt. This can be removed for plugins that want
# full control over build settings.
# apply_standard_settings(${PLUGIN_NAME})

# Symbols are hidden by default to reduce the chance of accidental conflicts
# between plugins. This should not be removed; any symbols that should be
# exported should be explicitly exported with the FLUTTER_PLUGIN_EXPORT macro.
set_target_properties(${PLUGIN_NAME} PROPERTIES
  CXX_VISIBILITY_PRESET hidden)
target_compile_definitions(${PLUGIN_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)

# Source include directories and library dependencies. Add any plugin-specific
# dependencies here.
target_include_directories(${PLUGIN_NAME} INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/include")

set(FLUTTER_LIBRARY "${EPHEMERAL_DIR}/libflutter_linux_gtk.so")



find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK REQUIRED gtk+-3.0)
list(APPEND FLUTTER_LIBRARY_HEADERS
  "fl_basic_message_channel.h"
  "fl_binary_codec.h"
  "fl_binary_messenger.h"
  "fl_dart_project.h"
  "fl_engine.h"
  "fl_json_message_codec.h"
  "fl_json_method_codec.h"
  "fl_message_codec.h"
  "fl_method_call.h"
  "fl_method_channel.h"
  "fl_method_codec.h"
  "fl_method_response.h"
  "fl_plugin_registrar.h"
  "fl_plugin_registry.h"
  "fl_standard_message_codec.h"
  "fl_standard_method_codec.h"
  "fl_string_codec.h"
  "fl_value.h"
  "fl_view.h"
  "flutter_linux.h"
)
list(PREPEND FLUTTER_LIBRARY_HEADERS "${EPHEMERAL_DIR}/flutter_linux/")
add_library(flutter INTERFACE)
target_include_directories(flutter INTERFACE
  "${EPHEMERAL_DIR}"
)
target_link_libraries(flutter INTERFACE "${FLUTTER_LIBRARY}")
target_link_libraries(flutter INTERFACE
  PkgConfig::GTK
  PkgConfig::GLIB
  PkgConfig::GIO
)
add_dependencies(flutter flutter_assemble)

# Print the GTK variables
message(STATUS "GTK_INCLUDE_DIRS: ${GTK_INCLUDE_DIRS}")
message(STATUS "GTK_LIBRARIES: ${GTK_LIBRARIES}")
# Include GTK directories and link against GTK libraries
target_include_directories(${PLUGIN_NAME} PRIVATE ${GTK_INCLUDE_DIRS})
target_link_libraries(${PLUGIN_NAME} PRIVATE ${GTK_LIBRARIES})
target_link_libraries(${PLUGIN_NAME} PRIVATE flutter)


if(!RustBuild)
  # List of absolute paths to libraries that should be bundled with the plugin.
  # This list could contain prebuilt libraries, or libraries created by an
  # external build triggered from this build file.
  set(flutter_gpu_texture_renderer_bundled_libraries
    ""
    PARENT_SCOPE
  )
endif ()

# === Tests ===
# These unit tests can be run from a terminal after building the example.

# Only enable test builds when building the example (which sets this variable)
# so that plugin clients aren't building the tests.
if (${include_${PROJECT_NAME}_tests})
if(${CMAKE_VERSION} VERSION_LESS "3.11.0")
message("Unit tests require CMake 3.11.0 or later")
else()
set(TEST_RUNNER "${PROJECT_NAME}_test")
enable_testing()

# Add the Google Test dependency.
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/release-1.11.0.zip
)
# Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
# Disable install commands for gtest so it doesn't end up in the bundle.
set(INSTALL_GTEST OFF CACHE BOOL "Disable installation of googletest" FORCE)

FetchContent_MakeAvailable(googletest)

# The plugin's exported API is not very useful for unit testing, so build the
# sources directly into the test binary rather than using the shared library.
add_executable(${TEST_RUNNER}
  test/flutter_gpu_texture_renderer_plugin_test.cc
  ${PLUGIN_SOURCES}
)
apply_standard_settings(${TEST_RUNNER})
target_include_directories(${TEST_RUNNER} PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}")
target_link_libraries(${TEST_RUNNER} PRIVATE flutter)
target_link_libraries(${TEST_RUNNER} PRIVATE PkgConfig::GTK)
target_link_libraries(${TEST_RUNNER} PRIVATE gtest_main gmock)

# Enable automatic test discovery.
include(GoogleTest)
gtest_discover_tests(${TEST_RUNNER})

endif()  # CMake version check
endif()  # include_${PROJECT_NAME}_tests