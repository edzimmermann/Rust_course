// Topic: Threads Channels and Synchronization
// Solution 08: rc_is_not_send
// Fix: Rc is not Send; use Arc for thread-safe reference counting.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <thread>
int main(){auto n=std::make_shared<int>(5);std::thread t([n]{std::cout<<*n<<"\n";});t.join();}
CPP_EQUIVALENT_END */

use std::{sync::Arc,thread};
fn main(){let n=Arc::new(5);let h=thread::spawn(move||println!("{n}"));h.join().unwrap();}
