% Interactive Rustc Matching Model
array[0..191] of var 0..1: our;
array[0..191] of var 0..1: rustc;

constraint abs(our[42] - rustc[41]) <= 2;
constraint abs(our[42] - rustc[88]) <= 2;
constraint abs(our[163] - rustc[162]) <= 1;

solve satisfy;
