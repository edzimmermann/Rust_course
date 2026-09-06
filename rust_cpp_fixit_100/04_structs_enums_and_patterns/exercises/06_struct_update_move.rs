// Topic: Structs Enums and Patterns
// Exercise 06: struct_update_move
// Goal: Struct update syntax moves owned fields
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct User{std::string name;bool active;};
int main(){User a{"Ada",true};User b=a;b.active=false;std::cout<<a.name<<" "<<b.name<<"\n";}
CPP_EQUIVALENT_END */

struct User{name:String,active:bool}
fn main(){let a=User{name:"Ada".into(),active:true};let b=User{active:false,..a};println!("{} {}",a.name,b.name);}
