### Monster Group FlatZinc Solver Target

add_library(monster_fzn OBJECT
  src/solvers/monster_fzn_solver.cpp
  src/solvers/monster_fzn_instance.cpp
  src/solvers/monster_mzn_solver.cpp
  src/solvers/monster_mzn_instance.cpp

  include/solvers/monster_fzn_solver.hh
  include/solvers/monster_fzn_instance.hh
  include/solvers/monster_mzn_solver.hh
  include/solvers/monster_mzn_instance.hh
)

target_include_directories(monster_fzn PRIVATE 
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
)

add_dependencies(monster_fzn monster_group)
