// Topic: Threads Channels and Synchronization
// Solution 03: mutex_lock_result
// Fix: Handle the LockResult before dereferencing the mutex guard.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <mutex>
int main(){int x=1;std::mutex m;{std::lock_guard<std::mutex> g(m);++x;}std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

use std::sync::Mutex;
fn main(){let m=Mutex::new(1);let mut g=m.lock().unwrap();*g+=1;println!("{}",*g);}
