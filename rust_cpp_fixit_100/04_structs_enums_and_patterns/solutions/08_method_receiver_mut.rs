// Topic: Structs Enums and Patterns
// Solution 08: method_receiver_mut
// Fix: Use &mut self and a mutable binding for a mutating method.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Counter{int i=0;void inc(){++i;}};
int main(){Counter c;c.inc();std::cout<<c.i<<"\n";}
CPP_EQUIVALENT_END */

struct Counter{i:i32}
impl Counter{fn inc(&mut self){self.i+=1;}}
fn main(){let mut c=Counter{i:0};c.inc();println!("{}",c.i);}
