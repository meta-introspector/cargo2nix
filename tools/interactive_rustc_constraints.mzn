% Interactive Rustc Partial Matching
array[0..191] of var 0..1: our;
array[0..191] of var 0..1: rustc;

constraint abs(our[42] - rustc[41]) <= 2;
constraint abs(our[156] - rustc[155]) <= 1;
constraint abs(our[89] - rustc[88]) <= 2;

solve satisfy;
