// Topic: Structs Enums and Patterns
// Solution 06: struct_update_move
// Fix: Clone before struct update if the original must remain fully usable.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct User{std::string name;bool active;};
int main(){User a{"Ada",true};User b=a;b.active=false;std::cout<<a.name<<" "<<b.name<<"\n";}
CPP_EQUIVALENT_END */

#[derive(Clone)] struct User{name:String,active:bool}
fn main(){let a=User{name:"Ada".into(),active:true};let b=User{active:false,..a.clone()};println!("{} {}",a.name,b.name);}
