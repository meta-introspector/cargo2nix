# ## Monster Group HiGHS Solver Target

add_library(monster_highs OBJECT
  src/solvers/monster_highs_solver.cpp
  src/solvers/monster_highs_wrapper.cpp

  include/solvers/monster_highs_solver.hh
  include/solvers/monster_highs_wrapper.hh
)

target_include_directories(monster_highs PRIVATE 
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

if(NOT HIGHS_PLUGIN)
  target_link_libraries(monster_highs PRIVATE highs::highs)
  target_link_libraries(monster_ffi highs::highs)
endif()

# ## Setup Monster Group HiGHS compilation
add_dependencies(monster_highs monster_group)
target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_HIGHS)
target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_highs>)
