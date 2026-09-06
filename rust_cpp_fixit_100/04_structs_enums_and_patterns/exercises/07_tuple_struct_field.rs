// Topic: Structs Enums and Patterns
// Exercise 07: tuple_struct_field
// Goal: Tuple struct indexing
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <tuple>
int main(){auto c=std::make_tuple(10,20,30);std::cout<<std::get<1>(c)<<"\n";}
CPP_EQUIVALENT_END */

struct Color(u8,u8,u8);
fn main(){let c=Color(10,20,30);println!("{}",c[1]);}
