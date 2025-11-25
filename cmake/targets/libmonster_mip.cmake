### Compile target for Monster Group base MIP interface

add_library(monster_mip OBJECT
  src/algorithms/monster_min_cut.cpp
  src/utils/monster_savestream.cpp

  src/solvers/MIP/monster_mip_instance.cpp

  include/solvers/MIP/monster_mip_wrap.hh
  include/solvers/MIP/monster_mip_instance.hh
  include/solvers/MIP/monster_mip_instance.hpp
)

target_include_directories(monster_mip PRIVATE 
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

add_dependencies(monster_mip monster_group)
