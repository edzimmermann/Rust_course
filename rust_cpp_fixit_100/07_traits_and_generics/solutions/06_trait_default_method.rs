// Topic: Traits and Generics
// Solution 06: trait_default_method
// Fix: Implement the required name method and inherit the default label method.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct Item{std::string n;std::string label()const{return "["+n+"]";}};
int main(){std::cout<<Item{"x"}.label()<<"\n";}
CPP_EQUIVALENT_END */

trait Named{fn name(&self)->&str;fn label(&self)->String{format!("[{}]",self.name())}}
struct Item{n:String}
impl Named for Item{fn name(&self)->&str{&self.n}}
fn main(){let i=Item{n:"x".into()};println!("{}",i.label());}
