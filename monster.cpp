#include "include/monster_ast.hh"
#include "include/monster_verifier.hh"
#include <iostream>
#include <vector>

int main(int argc, char* argv[]) {
    std::cout << "Monster Group Trait Verifier" << std::endl;
    
    if (argc < 2) {
        std::cout << "Usage: monster <trait1> [trait2] ..." << std::endl;
        return 1;
    }
    
    std::vector<MonsterGroup::TraitNode> traits;
    
    // Map command line traits to Monster Group elements
    for (int i = 1; i < argc; i++) {
        MonsterGroup::TraitNode trait;
        trait.name = argv[i];
        
        // Hash trait name to Monster Group element
        uint32_t hash = 0;
        for (char c : trait.name) {
            hash = hash * 31 + static_cast<uint32_t>(c);
        }
        trait.monster_element = hash % MonsterGroup::MONSTER_ORDER;
        trait.hecke_value = (trait.monster_element % 2 == 0) ? 
            MonsterGroup::HECKE_POS : MonsterGroup::HECKE_NEG;
        trait.hash_value = hash;
        
        traits.push_back(trait);
        
        std::cout << "Trait: " << trait.name 
                  << " -> Element: " << trait.monster_element
                  << " -> Hecke: " << trait.hecke_value << std::endl;
    }
    
    // Verify Monster Group constraints
    std::vector<uint64_t> elements;
    for (const auto& trait : traits) {
        elements.push_back(trait.monster_element);
    }
    
    bool valid = MonsterGroup::MonsterVerifier::verify_modular_constraint(elements);
    
    std::cout << "Monster Group verification: " 
              << (valid ? "PASSED" : "FAILED") << std::endl;
    
    return valid ? 0 : 1;
}
