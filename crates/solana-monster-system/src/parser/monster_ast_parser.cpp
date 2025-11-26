#include "../../include/monster_ast.hh"
#include <sstream>
#include <fstream>

extern MonsterGroup::TraitNode* parsed_trait;
extern int monster_yyparse();
extern void monster_yy_scan_string(const char* str);

namespace MonsterGroup {

MonsterAstParser::MonsterAstParser() {}

TraitNode MonsterAstParser::parse_trait_string(const std::string& trait_str) {
    // Simple string parsing for Monster Group traits
    TraitNode trait;
    
    std::istringstream iss(trait_str);
    std::string token;
    
    // Parse format: "trait_name element:123 hecke:456 hash:789"
    if (iss >> trait.name) {
        std::string key;
        while (iss >> key) {
            if (key.find("element:") == 0) {
                trait.monster_element = std::stoull(key.substr(8));
            } else if (key.find("hecke:") == 0) {
                trait.hecke_value = std::stoi(key.substr(6));
            } else if (key.find("hash:") == 0) {
                trait.hash_value = std::stoul(key.substr(5));
            }
        }
    }
    
    return trait;
}

std::vector<TraitNode> MonsterAstParser::parse_trait_file(const std::string& filename) {
    std::vector<TraitNode> traits;
    std::ifstream file(filename);
    std::string line;
    
    while (std::getline(file, line)) {
        if (!line.empty() && line[0] != '#') {
            traits.push_back(parse_trait_string(line));
        }
    }
    
    return traits;
}

TraitNode MonsterAstParser::create_trait_from_name(const std::string& name) {
    TraitNode trait;
    trait.name = name;
    
    // Hash trait name to Monster Group element
    uint32_t hash = 0;
    for (char c : name) {
        hash = hash * 31 + static_cast<uint32_t>(c);
    }
    
    trait.monster_element = hash % MONSTER_ORDER;
    trait.hecke_value = (trait.monster_element % 2 == 0) ? HECKE_POS : HECKE_NEG;
    trait.hash_value = hash;
    
    return trait;
}

bool MonsterAstParser::validate_trait(const TraitNode& trait) {
    return trait.monster_element < MONSTER_ORDER &&
           (trait.hecke_value == HECKE_POS || trait.hecke_value == HECKE_NEG) &&
           !trait.name.empty();
}

} // namespace MonsterGroup
