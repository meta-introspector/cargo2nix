# Define the monster_c_wrapper library
add_library(monster_c_wrapper SHARED
    ${PROJECT_SOURCE_DIR}/src/monster_ffi.cpp
    ${MONSTER_EXPRESSION_SOURCES}
)

# Link against monster_ffi (the main Monster Group library)
target_link_libraries(monster_c_wrapper PRIVATE monster_ffi)

# Set include directories for the C wrapper
target_include_directories(monster_c_wrapper PRIVATE
    ${PROJECT_SOURCE_DIR}/include
    ${PROJECT_BINARY_DIR}/include
    ${PROJECT_SOURCE_DIR}/src/expressions
)

# Set RPATH for the C wrapper to find libmonster_ffi.so at runtime
target_link_options(monster_c_wrapper PRIVATE
    -Wl,-rpath,\$ORIGIN/../lib
    -Wl,-rpath,${CMAKE_INSTALL_PREFIX}/lib
)

# Install the C wrapper library
install(TARGETS monster_c_wrapper
    DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
