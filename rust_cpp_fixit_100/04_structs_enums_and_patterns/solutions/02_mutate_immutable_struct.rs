// Topic: Structs Enums and Patterns
// Solution 02: mutate_immutable_struct
// Fix: Make the struct binding mutable.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Point{int x;int y;};
int main(){Point p{1,2};p.x=5;std::cout<<p.x<<"\n";}
CPP_EQUIVALENT_END */

struct Point{x:i32,y:i32}
fn main(){let mut p=Point{x:1,y:2};p.x=5;println!("{}",p.x);}
