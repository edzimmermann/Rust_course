// Topic: Structs Enums and Patterns
// Exercise 08: method_receiver_mut
// Goal: Mutable method receivers
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Counter{int i=0;void inc(){++i;}};
int main(){Counter c;c.inc();std::cout<<c.i<<"\n";}
CPP_EQUIVALENT_END */

struct Counter{i:i32}
impl Counter{fn inc(&self){self.i+=1;}}
fn main(){let c=Counter{i:0};c.inc();}
