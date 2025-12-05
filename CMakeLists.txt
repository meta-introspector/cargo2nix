cmake_minimum_required(VERSION 3.16)
project(monster_group_cargo2nix VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Gecode
find_package(PkgConfig REQUIRED)
pkg_check_modules(GECODE gecode)

# Include Monster Group targets
include(cmake/targets/monster_expression_sources.cmake)
include(cmake/targets/libmonster_group.cmake)
include(cmake/targets/libmonster_gecode.cmake)
include(cmake/targets/monster_c_wrapper.cmake)
include(cmake/targets/monster.cmake)

# Build configuration
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

# Compiler flags
set(CMAKE_CXX_FLAGS_RELEASE "-O3 -DNDEBUG")
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0")

# Version configuration
set(libmonster_VERSION ${PROJECT_VERSION})

# Configure template variables
set(CONF_INCLUDE_DIRS "${CMAKE_INSTALL_PREFIX}/include")
set(CONF_DEPENDENCIES "")
if(GECODE_FOUND)
  set(CONF_DEPENDENCIES "find_dependency(PkgConfig)\npkg_check_modules(GECODE gecode)")
endif()

# Generate config files
include(CMakePackageConfigHelpers)
configure_file(
  cmake/templates/libmonsterConfig.cmake.in
  ${CMAKE_CURRENT_BINARY_DIR}/libmonsterConfig.cmake
  @ONLY
)

write_basic_package_version_file(
  ${CMAKE_CURRENT_BINARY_DIR}/libmonsterConfigVersion.cmake
  VERSION ${libmonster_VERSION}
  COMPATIBILITY SameMajorVersion
)

# Install targets
install(TARGETS monster_ffi monster_c_wrapper monster
  EXPORT libmonsterTargets
  LIBRARY DESTINATION lib
  ARCHIVE DESTINATION lib
  RUNTIME DESTINATION bin
)

install(EXPORT libmonsterTargets
  FILE libmonsterTargets.cmake
  DESTINATION lib/cmake/libmonster
)

install(FILES
  ${CMAKE_CURRENT_BINARY_DIR}/libmonsterConfig.cmake
  ${CMAKE_CURRENT_BINARY_DIR}/libmonsterConfigVersion.cmake
  DESTINATION lib/cmake/libmonster
)

install(DIRECTORY include/
  DESTINATION include
  FILES_MATCHING PATTERN "*.hh"
)
