### Monster Group Gecode Solver Target

if(GECODE_FOUND)

  ### Compile target for Monster Group Gecode interface
  add_library(monster_gecode OBJECT
    src/solvers/monster_gecode_space.cpp
    src/solvers/monster_gecode_constraints.cpp
    src/solvers/monster_gecode_solver.cpp

    include/solvers/monster_gecode_space.hh
    include/solvers/monster_gecode_constraints.hh
    include/solvers/monster_gecode_solver.hh
  )
  
  target_include_directories(monster_gecode PRIVATE 
    "${GECODE_INCLUDE_DIRS}"
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
  )
  
  add_dependencies(monster_gecode monster_group)

  ### Setup Monster Group compilation
  target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_GECODE)
  target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_gecode>)

  target_link_libraries(monster_ffi 
    Gecode::Driver 
    Gecode::Int 
    Gecode::Kernel 
    Gecode::Search
  )

endif()
