// Topic: Traits and Generics
// Exercise 02: derive_debug
// Goal: Derive common traits
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Point{int i;};
std::ostream& operator<<(std::ostream& os,const Point& p){return os<<"Point{"<<p.i<<"}";}
int main(){std::cout<<Point{3}<<"\n";}
CPP_EQUIVALENT_END */

struct Point{i:i32}
fn main(){println!("{:?}",Point{i:3});}
