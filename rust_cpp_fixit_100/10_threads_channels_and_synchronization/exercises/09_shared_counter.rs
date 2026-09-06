// Topic: Threads Channels and Synchronization
// Exercise 09: shared_counter
// Goal: Arc Mutex counter
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <mutex>
#include <thread>
int main(){int n=0;std::mutex m;std::thread t([&]{std::lock_guard<std::mutex> g(m);++n;});t.join();std::cout<<n<<"\n";}
CPP_EQUIVALENT_END */

use std::{sync::Mutex,thread};
fn main(){let n=Mutex::new(0);let h=thread::spawn(move||*n.lock().unwrap()+=1);h.join().unwrap();println!("{}",*n.lock().unwrap());}
