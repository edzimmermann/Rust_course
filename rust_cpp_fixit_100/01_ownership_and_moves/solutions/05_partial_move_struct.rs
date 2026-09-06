// Topic: Ownership and Moves
// Solution 05: partial_move_struct
// Fix: Clone the field if the whole struct still needs to be used.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct User{ std::string name; unsigned age; };
int main(){ User u{"Lin",42}; auto name=u.name; std::cout<<u.name<<" "<<u.age<<"\n"<<name<<"\n"; }
CPP_EQUIVALENT_END */

struct User{ name:String, age:u8 }
fn main(){ let u=User{name:"Lin".into(),age:42}; let name=u.name.clone(); println!("{} {}",u.name,u.age); println!("{name}"); }
