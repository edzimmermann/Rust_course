// Topic: Traits and Generics
// Solution 05: associated_function_call
// Fix: Call an associated function with Type::function syntax.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct User{std::string name;static User make(std::string n){return User{std::move(n)};}};
int main(){auto v=User::make("Ada");std::cout<<v.name<<"\n";}
CPP_EQUIVALENT_END */

struct User{name:String}
impl User{fn new(name:&str)->Self{Self{name:name.into()}}}
fn main(){let v=User::new("Ada");println!("{}",v.name);}
