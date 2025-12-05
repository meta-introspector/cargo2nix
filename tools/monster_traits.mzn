% Monster Group Trait Mapping
int: n_traits = 192;
array[1..n_traits] of var 0..1: trait_used;
% Block monster_triple_db_loader.rs consumes fs, HashMap and produces ContentID(String);, GitModuleDB, GitModule, CargoCrateDB, CargoCrate, ASTDeclDB, ASTDecl, MonsterTripleDB
constraint trait_used[181] = 1; % Monster index for monster_triple_db_loader.rs
% Block solana_submodule_driver.rs consumes fs, Path, Command, {HashMap, HashSet} and produces SolanaSubmoduleDriver, CrateInfo
constraint trait_used[9] = 1; % Monster index for solana_submodule_driver.rs
% Block complete_monster_system.rs consumes fs, HashMap and produces CompleteMonsterSystem, GitRepo, CargoCrate, ASTDecl
constraint trait_used[164] = 1; % Monster index for complete_monster_system.rs
% Block simple_monster_traits.rs consumes MonsterSignature, * and produces 
constraint trait_used[122] = 1; % Monster index for simple_monster_traits.rs
solve minimize sum(trait_used);