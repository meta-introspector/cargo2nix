% Monster Group Trait Test Model
int: num_traits = 3;
int: monster_order = 196883;

array[1..num_traits] of var 0..monster_order-1: monster_elements;
array[1..num_traits] of var {-5472, 196883}: hecke_values;
var 1..num_traits: selected_index;

constraint all_different(monster_elements);
constraint sum(i in 1..num_traits) (monster_elements[i]) mod 24 = 0;

solve minimize monster_elements[selected_index];

output [
    "monster_elements = ", show(monster_elements), "\n",
    "hecke_values = ", show(hecke_values), "\n",
    "selected_trait = trait_", show(selected_index), "\n"
];
