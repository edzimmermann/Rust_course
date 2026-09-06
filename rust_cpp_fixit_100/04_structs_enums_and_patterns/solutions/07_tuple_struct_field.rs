// Topic: Structs Enums and Patterns
// Solution 07: tuple_struct_field
// Fix: Access tuple-struct fields with .0, .1, etc.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <tuple>
int main(){auto c=std::make_tuple(10,20,30);std::cout<<std::get<1>(c)<<"\n";}
CPP_EQUIVALENT_END */

struct Color(u8,u8,u8);
fn main(){let c=Color(10,20,30);println!("{}",c.1);}
