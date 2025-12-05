### Monster Group SCIP Solver Target

if (NOT DEFINED EMSCRIPTEN AND SCIP_FOUND)
	### Compile target for Monster Group SCIP interface
	add_library(monster_scip OBJECT
		src/solvers/monster_scip_solver.cpp
		src/solvers/monster_scip_wrapper.cpp

		include/solvers/monster_scip_solver.hh
		include/solvers/monster_scip_wrapper.hh
	)
	
	target_include_directories(monster_scip PRIVATE 
		"${CMAKE_CURRENT_SOURCE_DIR}/include"
	)
	
	add_dependencies(monster_scip monster_group)

	### Setup Monster Group SCIP compilation
  	target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_SCIP)
	target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_scip>)
	target_link_libraries(monster_ffi ${SCIP_LIBRARIES})
endif()
