### Monster Group Chuffed Solver Target

if(CHUFFED_FOUND)

  ### Compile target for Monster Group Chuffed interface
  add_library(monster_chuffed OBJECT
    src/solvers/monster_chuffed_solver.cpp
    src/solvers/monster_chuffed_wrapper.cpp

    include/solvers/monster_chuffed_solver.hh
    include/solvers/monster_chuffed_wrapper.hh
  )
  
  target_include_directories(monster_chuffed PRIVATE 
    "${CHUFFED_INCLUDE_DIRS}"
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
  )
  
  target_compile_definitions(monster_chuffed PRIVATE CHUFFED_VERSION="${chuffed_VERSION}")
  add_dependencies(monster_chuffed monster_group)

  ### Setup Monster Group Chuffed compilation
  target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_CHUFFED)
  target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_chuffed>)
  target_link_libraries(monster_ffi ${CHUFFED_LIBRARIES})

  ### Copy Monster Group library for Chuffed
  file(COPY "${CMAKE_CURRENT_SOURCE_DIR}/share/monster/" DESTINATION "${CMAKE_BINARY_DIR}/share/monster/chuffed_internal")
  install(
    DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/share/monster/"
    DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/monster/chuffed_internal"
  )

else()

  ### Remove Monster Group Chuffed library if not present
  file(REMOVE_RECURSE "${CMAKE_BINARY_DIR}/share/monster/chuffed_internal")

endif()
