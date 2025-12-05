### Monster Group Atlantis Solver Target

if(atlantis_FOUND)

  ### Compile target for Monster Group Atlantis interface
  add_library(monster_atlantis OBJECT
    src/solvers/monster_atlantis_solver.cpp
    src/solvers/monster_atlantis_wrapper.cpp

    include/solvers/monster_atlantis_solver.hh
    include/solvers/monster_atlantis_wrapper.hh
  )
  
  target_link_libraries(monster_atlantis PRIVATE atlantis::atlantis)
  target_compile_definitions(monster_atlantis PRIVATE ATLANTIS_VERSION="${atlantis_FOUND}")
  target_include_directories(monster_atlantis PRIVATE 
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
  )
  
  add_dependencies(monster_atlantis monster_group)

  ### Setup Monster Group Atlantis compilation
  target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_ATLANTIS)
  target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_atlantis>)
  target_link_libraries(monster_ffi atlantis::atlantis)

  ### Copy Monster Group library for Atlantis
  file(COPY "${CMAKE_CURRENT_SOURCE_DIR}/share/monster/" DESTINATION "${CMAKE_BINARY_DIR}/share/monster/atlantis_internal")
  install(
    DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/share/monster/"
    DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/monster/atlantis_internal"
  )

else()

  ### Remove Monster Group Atlantis library if not present
  file(REMOVE_RECURSE "${CMAKE_BINARY_DIR}/share/monster/atlantis_internal")

endif()
