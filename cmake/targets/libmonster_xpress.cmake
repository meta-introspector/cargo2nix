### Monster Group FICO Xpress Solver Target

if (NOT DEFINED EMSCRIPTEN AND XPRESS_FOUND)
	### Compile target for Monster Group Xpress interface
	add_library(monster_xpress OBJECT
		src/solvers/monster_xpress_solver.cpp
		src/solvers/monster_xpress_wrapper.cpp

		include/solvers/monster_xpress_solver.hh
		include/solvers/monster_xpress_wrapper.hh
	)
	
	target_include_directories(monster_xpress PRIVATE 
		"${CMAKE_CURRENT_SOURCE_DIR}/include"
	)
	
	add_dependencies(monster_xpress monster_group)

	### Setup Monster Group Xpress compilation
  	target_compile_definitions(monster_ffi PRIVATE HAS_MONSTER_XPRESS)
	target_sources(monster_ffi PRIVATE $<TARGET_OBJECTS:monster_xpress>)
	target_link_libraries(monster_ffi ${XPRESS_LIBRARIES})
endif()
