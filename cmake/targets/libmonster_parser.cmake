# -------------------------------------------------------------------------------------------------------------------
## Monster Group Parser Generation Targets

find_package(BISON 3.4)
find_package(FLEX 2.5)

if(BISON_FOUND AND FLEX_FOUND)
  BISON_TARGET(MonsterParser
    ${PROJECT_SOURCE_DIR}/lib/monster_parser.yxx
    ${PROJECT_BINARY_DIR}/monster_parser.tab.cpp
    DEFINES_FILE ${PROJECT_BINARY_DIR}/include/monster_parser.tab.hh
    COMPILE_FLAGS "-p monster_yy -l -Werror"
  )

  FLEX_TARGET(MonsterLexer
    ${PROJECT_SOURCE_DIR}/lib/monster_lexer.lxx
    ${PROJECT_BINARY_DIR}/monster_lexer.yy.cpp
    COMPILE_FLAGS "-P monster_yy -L"
  )
  ADD_FLEX_BISON_DEPENDENCY(MonsterLexer MonsterParser)
else()
  # Use cached files if parser generators not available
  set(BISON_MonsterParser_OUTPUTS
    ${PROJECT_SOURCE_DIR}/lib/cached/monster_parser.tab.cpp
    ${PROJECT_SOURCE_DIR}/lib/cached/monster_parser.tab.hh
  )
  set(FLEX_MonsterLexer_OUTPUTS ${PROJECT_SOURCE_DIR}/lib/cached/monster_lexer.yy.cpp)
endif()

add_library(monster_parser OBJECT
  ${BISON_MonsterParser_OUTPUTS}
  ${FLEX_MonsterLexer_OUTPUTS}
  src/parser/monster_ast_parser.cpp
)

target_include_directories(monster_parser PRIVATE 
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
  "${PROJECT_BINARY_DIR}/include"
)

set_target_properties(monster_parser PROPERTIES
  CXX_CLANG_TIDY ""
)
