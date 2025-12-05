### Monster Group OsiCBC Solver Target

if(OSICBC_FOUND)

  ### Compile target for Monster Group OsiCBC interface
  add_library(monster_osicbc OBJECT
    src/solvers/monster_osicbc_solver.cpp
    src/solvers/monster_cbc_wrapper.cpp

    include/solvers/monster_osicbc_solver.hh
    include/solvers/monster_cbc_wrapper.hh
  )
  
  target_include_directories(monster_osicbc PRIVATE 
    ${OSICBC_INCLUDE_DIRS}
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
  )
  
  add_dependencies(monster_osicbc monster_group)
  
  if (UNIX AND NOT WIN32)
    target_compile_definitions(monster_osicbc PRIVATE HAVE_CONFIG_H)
  endif()

  ### Setup Monster Group OsiCBC compilation
  target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_OSICBC)
  target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_osicbc>)
  target_link_libraries(monster_ffi ${OSICBC_TARGETS})

endif()
