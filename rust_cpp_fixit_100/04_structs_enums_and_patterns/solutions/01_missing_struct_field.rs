// Topic: Structs Enums and Patterns
// Solution 01: missing_struct_field
// Fix: Initialize every struct field.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Point{int x;int y;};
int main(){Point p{3,0};std::cout<<p.x<<" "<<p.y<<"\n";}
CPP_EQUIVALENT_END */

struct Point{x:i32,y:i32}
fn main(){let p=Point{x:3,y:0};println!("{} {}",p.x,p.y);}
