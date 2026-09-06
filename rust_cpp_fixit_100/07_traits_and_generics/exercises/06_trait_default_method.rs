// Topic: Traits and Generics
// Exercise 06: trait_default_method
// Goal: Using a default method
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
struct Item{std::string n;std::string label()const{return "["+n+"]";}};
int main(){std::cout<<Item{"x"}.label()<<"\n";}
CPP_EQUIVALENT_END */

trait Named{fn name(&self)->&str;fn label(&self)->String{format!("[{}]",self.name())}}
struct Item{n:String}
impl Named for Item{fn label(&self)->String{self.n.clone()}}
fn main(){let i=Item{n:"x".into()};println!("{}",i.label());}
