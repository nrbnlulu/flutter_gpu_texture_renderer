# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "flutter_gpu_texture_renderer")
project(${PROJECT_NAME} LANGUAGES C)


option(RustBuild "RustBuild" ON)
set(EPHEMERAL_DIR "/home/dev/Documents/flutter_linux_3.24.0-stable/flutter/bin/cache/artifacts/engine/linux-x64" CACHE PATH "Path to the ephemeral directory")
function(list_transform_prepend var prefix)
  set(temp "")
  foreach(f ${${var}})
    list(APPEND temp "${prefix}${f}")
  endforeach()
  set(${var} "${temp}" PARENT_SCOPE)
endfunction()
if(RustBuild)
  message("ephemeral is ${EPHEMERAL_DIR}")
  if(NOT EPHEMERAL_DIR)
    message(FATAL_ERROR "EPHEMERAL_DIR must be set when building with Rust")
  endif()
  # ---------- mostly copied from the generated cmake of example/linux/build/flutter/CMakeLists.txt ----------
  # === Flutter Library ===
  # System-level dependencies.
  find_package(PkgConfig REQUIRED)
  pkg_check_modules(GTK REQUIRED IMPORTED_TARGET gtk+-3.0)
  pkg_check_modules(GLIB REQUIRED IMPORTED_TARGET glib-2.0)
  pkg_check_modules(GIO REQUIRED IMPORTED_TARGET gio-2.0)

  set(FLUTTER_LIBRARY "${EPHEMERAL_DIR}/libflutter_linux_gtk.so")


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
  list_transform_prepend(FLUTTER_LIBRARY_HEADERS "${EPHEMERAL_DIR}/flutter_linux/")

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

  add_custom_target(flutter_assemble DEPENDS
    "${FLUTTER_LIBRARY}"
    ${FLUTTER_LIBRARY_HEADERS}
  )
  # ------------------------------------------------- END ---------------------------------------------------------------------
  # ---------- mostly copied from the generated ./linux/CMakeLists.txt of flutter plugin ----------
  # This value is used when generating builds using this plugin, so it must
  # not be changed.
  set(PLUGIN_NAME "flutter_gpu_texture_renderer_plugin")

  # Any new source files that you add to the plugin should be added here.
  list(APPEND PLUGIN_SOURCES
    "api.c"
  )

  # Define the plugin library target. Its name must not be changed (see comment
  # on PLUGIN_NAME above).
  add_library(${PLUGIN_NAME} SHARED
    ${PLUGIN_SOURCES}
  )

  # # Symbols are hidden by default to reduce the chance of accidental conflicts
  # # between plugins. This should not be removed; any symbols that should be
  # # exported should be explicitly exported with the FLUTTER_PLUGIN_EXPORT macro.
  # set_target_properties(${PLUGIN_NAME} PROPERTIES
  #   CXX_VISIBILITY_PRESET hidden)
  target_compile_definitions(${PLUGIN_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)

  # Source include directories and library dependencies. Add any plugin-specific
  # dependencies here.
  target_include_directories(${PLUGIN_NAME} INTERFACE
    "${CMAKE_CURRENT_SOURCE_DIR}/include")

  target_include_directories(${PLUGIN_NAME} PRIVATE
    "${EPHEMERAL_DIR}"
  )
  target_link_libraries(${PLUGIN_NAME} PRIVATE flutter)
  target_link_libraries(${PLUGIN_NAME} PRIVATE PkgConfig::GTK)
  add_dependencies(${PLUGIN_NAME} flutter_assemble)


  # ------------------------------------------------- END ---------------------------------------------------------------------


else()

endif()
