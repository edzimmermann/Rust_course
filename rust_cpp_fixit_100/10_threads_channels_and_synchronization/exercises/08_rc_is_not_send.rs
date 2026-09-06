// Topic: Threads Channels and Synchronization
// Exercise 08: rc_is_not_send
// Goal: Use Arc across threads
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <thread>
int main(){auto n=std::make_shared<int>(5);std::thread t([n]{std::cout<<*n<<"\n";});t.join();}
CPP_EQUIVALENT_END */

use std::{rc::Rc,thread};
fn main(){let n=Rc::new(5);let h=thread::spawn(move||println!("{n}"));h.join().unwrap();}
