### Monster Group Geas Solver Target

if(GEAS_FOUND)

  ### Compile target for Monster Group Geas interface
  add_library(monster_geas OBJECT
    src/solvers/monster_geas_constraints.cpp
    src/solvers/monster_geas_solver.cpp
    src/solvers/monster_geas_wrapper.cpp

    include/solvers/monster_geas_constraints.hh
    include/solvers/monster_geas_solver.hh
    include/solvers/monster_geas_wrapper.hh
  )
  
  target_include_directories(monster_geas PRIVATE 
    "${GEAS_INCLUDE_DIRS}"
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
  )
  
  add_dependencies(monster_geas monster_group)

  ### Setup Monster Group Geas compilation
  target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_GEAS)
  target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_geas>)
  target_link_libraries(monster_ffi Geas)

endif()
