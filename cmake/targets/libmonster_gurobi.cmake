### Monster Group Gurobi Solver Target

if (NOT DEFINED EMSCRIPTEN AND GUROBI_FOUND)
	### Compile target for Monster Group Gurobi interface
	add_library(monster_gurobi OBJECT
		src/solvers/monster_gurobi_solver.cpp
		src/solvers/monster_gurobi_wrapper.cpp

		include/solvers/monster_gurobi_solver.hh
		include/solvers/monster_gurobi_wrapper.hh
	)
	
	target_include_directories(monster_gurobi PRIVATE 
		"${CMAKE_CURRENT_SOURCE_DIR}/include"
	)
	
	add_dependencies(monster_gurobi monster_group)

	### Setup Monster Group Gurobi compilation
  	target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_GUROBI)
	target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_gurobi>)
	target_link_libraries(monster_ffi ${GUROBI_LIBRARIES})
endif()
