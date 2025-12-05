### Monster Group Verification Library Target

add_library(monster_group OBJECT
  src/monster_ffi.cpp
  src/monster_verifier.cpp
  src/trait_mapper.cpp
  ${MONSTER_EXPRESSION_SOURCES}

  include/monster_ast.hh
  include/monster_verifier.hh
  include/trait_mapper.hh
)

target_include_directories(monster_group PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Monster Group FFI shared library
add_library(monster_ffi SHARED
  $<TARGET_OBJECTS:monster_group>
)

target_link_libraries(monster_ffi PUBLIC monster_group)
