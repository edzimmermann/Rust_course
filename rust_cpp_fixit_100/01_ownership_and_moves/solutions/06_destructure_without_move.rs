// Topic: Ownership and Moves
// Solution 06: destructure_without_move
// Fix: Bind the field by reference with ref to avoid moving it out.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct Pair{std::string left,right;};
int main(){ Pair p{"L","R"}; const auto& left=p.left; std::cout<<p.right<<"\n"<<left<<"\n"; }
CPP_EQUIVALENT_END */

struct Pair{ left:String,right:String }
fn main(){ let p=Pair{left:"L".into(),right:"R".into()}; let Pair{ref left,..}=p; println!("{}",p.right); println!("{left}"); }
