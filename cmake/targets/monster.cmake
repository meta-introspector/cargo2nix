#### Monster Group Executable Target

add_executable(monster monster.cpp)
target_link_libraries(monster monster_ffi)

install(
  TARGETS monster
  EXPORT libmonsterTargets
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
