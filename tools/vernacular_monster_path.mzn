
% Vernacular → Monster Path Solver
int: n_steps = 5;
array[1..n_steps] of var 0..191: path;

% Input: vernacular embeddings (simplified as start points)
var 0..191: vernacular_struct;    % struct embedding
var 0..191: vernacular_function;  % function embedding  
var 0..191: vernacular_trait;     % trait embedding
var 0..191: vernacular_impl;      % impl embedding

% Output: monster targets
int: monster_struct = 42;
int: monster_function = 156;
int: monster_trait = 89;
int: monster_impl = 127;

% Path constraints: vernacular → monster
constraint path[1] = vernacular_struct;
constraint path[n_steps] = monster_struct;

% Alternative paths for different constructs
constraint vernacular_struct = 40;     % close to monster 42
constraint vernacular_function = 154;  % close to monster 156
constraint vernacular_trait = 87;      % close to monster 89
constraint vernacular_impl = 125;      % close to monster 127

% Path must be continuous (small jumps)
constraint forall(i in 1..n_steps-1)(
    abs(path[i+1] - path[i]) <= 10
);

% Objective: minimize path distance
solve minimize sum(i in 1..n_steps-1)(abs(path[i+1] - path[i]));
