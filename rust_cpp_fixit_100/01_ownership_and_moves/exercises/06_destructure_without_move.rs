// Topic: Ownership and Moves
// Exercise 06: destructure_without_move
// Goal: Destructuring owned fields
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct Pair{std::string left,right;};
int main(){ Pair p{"L","R"}; const auto& left=p.left; std::cout<<p.right<<"\n"<<left<<"\n"; }
CPP_EQUIVALENT_END */

struct Pair{ left:String,right:String }
fn main(){ let p=Pair{left:"L".into(),right:"R".into()}; let Pair{left,..}=p; println!("{} {}",p.left,p.right); println!("{left}"); }
