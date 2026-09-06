// Topic: Traits and Generics
// Solution 10: trait_object_dyn
// Fix: Use dyn for a trait object type.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Speak{virtual const char* speak()const=0;virtual ~Speak()=default;};struct Dog:Speak{const char* speak()const override{return "woof";}};
void say(const Speak& x){std::cout<<x.speak()<<"\n";}int main(){Dog d;say(d);}
CPP_EQUIVALENT_END */

trait Speak{fn speak(&self)->&str;}
struct Dog;impl Speak for Dog{fn speak(&self)->&str{"woof"}}
fn say(x:&dyn Speak){println!("{}",x.speak());}
fn main(){say(&Dog);}
