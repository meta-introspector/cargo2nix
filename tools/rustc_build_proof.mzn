% Monster Protocol Build Proof
int: n_components = 4; % rustc components
array[1..n_components] of var 0..191: component_monsters;

% Rustc component Monster indices
constraint component_monsters[1] = 41;  % rustc_driver
constraint component_monsters[2] = 88;  % rustc_interface
constraint component_monsters[3] = 162; % rustc_middle
constraint component_monsters[4] = 94;  % rustc_codegen_llvm

% Compatibility constraints
constraint forall(i in 1..n_components-1)(
    abs(component_monsters[i+1] - component_monsters[i]) <= 50
);

% Build path must be valid
constraint sum(component_monsters) > 0;

solve satisfy;