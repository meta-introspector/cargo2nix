#pragma once

#include "monster_ast.hh"
#include <string>
#include <vector>

namespace MonsterGroup {

class MonsterAstParser {
public:
    MonsterAstParser();
    
    TraitNode parse_trait_string(const std::string& trait_str);
    std::vector<TraitNode> parse_trait_file(const std::string& filename);
    TraitNode create_trait_from_name(const std::string& name);
    bool validate_trait(const TraitNode& trait);
};

} // namespace MonsterGroup
