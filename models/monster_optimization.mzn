% Monster Group Optimization Model
int: elliptic_fiber;
int: torus_x;
int: torus_y;
int: monster_stabilizer;

var 0..23: x;
var 0..23: y;

constraint (x + y + elliptic_fiber) mod 24 = 0;
constraint x * torus_x + y * torus_y >= monster_stabilizer;

var int: objective = x * x + y * y;
solve minimize objective;

output ["{\"x\": \(x), \"y\": \(y), \"objective\": \(objective)}"];
