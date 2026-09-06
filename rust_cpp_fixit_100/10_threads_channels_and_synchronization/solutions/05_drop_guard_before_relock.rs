// Topic: Threads Channels and Synchronization
// Solution 05: drop_guard_before_relock
// Fix: End or drop the first guard before locking the same non-reentrant mutex again.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <mutex>
int main(){int x=0;std::mutex m;{std::lock_guard<std::mutex> g(m);std::cout<<x<<"\n";}{std::lock_guard<std::mutex> g(m);++x;}std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

use std::sync::Mutex;
fn main(){let m=Mutex::new(0);{let g=m.lock().unwrap();println!("{}",*g);}let mut g2=m.lock().unwrap();*g2+=1;println!("{}",*g2);}
