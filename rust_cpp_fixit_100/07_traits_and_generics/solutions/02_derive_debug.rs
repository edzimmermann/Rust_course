// Topic: Traits and Generics
// Solution 02: derive_debug
// Fix: Derive Debug for a type printed with {:?}.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Point{int i;};
std::ostream& operator<<(std::ostream& os,const Point& p){return os<<"Point{"<<p.i<<"}";}
int main(){std::cout<<Point{3}<<"\n";}
CPP_EQUIVALENT_END */

#[derive(Debug)] struct Point{i:i32}
fn main(){println!("{:?}",Point{i:3});}
