### Monster Group CPLEX Solver Target

if (NOT DEFINED EMSCRIPTEN AND CPLEX_FOUND)
	### Compile target for Monster Group CPLEX interface
	add_library(monster_cplex OBJECT
		src/solvers/monster_cplex_solver.cpp
		src/solvers/monster_mip_wrapper.cpp

		include/solvers/monster_cplex_solver.hh
		include/solvers/monster_mip_wrapper.hh
	)

	if(NOT CPLEX_PLUGIN)
		target_include_directories(monster_cplex PRIVATE 
			${CPLEX_INCLUDE_DIRS}
			"${CMAKE_CURRENT_SOURCE_DIR}/include"
		)
		target_link_libraries(monster_ffi ${CPLEX_LIBRARIES})
		set_target_properties(monster_cplex PROPERTIES COMPILE_FLAGS ${CPLEX_COMPILE_FLAGS})
		set_target_properties(monster_ffi PROPERTIES COMPILE_FLAGS ${CPLEX_COMPILE_FLAGS})
	endif()

	### Setup Monster Group CPLEX compilation
	add_dependencies(monster_cplex monster_group)
	target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_CPLEX)
	target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_cplex>)
endif()
