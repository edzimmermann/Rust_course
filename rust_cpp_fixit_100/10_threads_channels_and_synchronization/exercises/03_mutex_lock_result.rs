// Topic: Threads Channels and Synchronization
// Exercise 03: mutex_lock_result
// Goal: Mutex::lock returns Result
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <mutex>
int main(){int x=1;std::mutex m;{std::lock_guard<std::mutex> g(m);++x;}std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

use std::sync::Mutex;
fn main(){let m=Mutex::new(1);let mut g=m.lock();*g+=1;}
