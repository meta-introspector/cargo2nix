% Complete Monster Protocol Proof
int: n_monsters = 192;
array[0..n_monsters-1] of var 0..1: monster_used;
constraint monster_used[88] = 1; % rustc_rustc_interface_Input
constraint monster_used[88] = 1; % rustc_rustc_interface_Config
constraint monster_used[23] = 1; % trait_use collectionsHashMap;Trait
constraint monster_used[0] = 1; % trait_fsTrait
constraint monster_used[92] = 1; % trait_processTrait
constraint monster_used[41] = 1; % tool_self_describing_monster.rs_StructType
constraint monster_used[105] = 1; % tool_zk_program_prover.rs_FunctionType
constraint monster_used[88] = 1; % rustc_rustc_interface_CompilerResult
constraint monster_used[41] = 1; % rustc_rustc_driver_Config
constraint monster_used[69] = 1; % trait_envTrait
constraint monster_used[46] = 1; % trait_collectionsHashMapTrait
constraint monster_used[105] = 1; % tool_zk_program_prover.rs_StructType
constraint monster_used[41] = 1; % tool_self_describing_monster.rs_FunctionType
constraint monster_used[41] = 1; % rustc_rustc_driver_Args
constraint monster_used[41] = 1; % rustc_rustc_driver_ExitCode
constraint abs(monster_used[41] - monster_used[41]) <= 1; % rustc_driver compatible with self_describing_monster.rs
solve minimize sum(monster_used);