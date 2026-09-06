// Topic: Threads Channels and Synchronization
// Solution 09: shared_counter
// Fix: Wrap the Mutex in Arc when both the worker and caller need ownership.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <mutex>
#include <thread>
int main(){int n=0;std::mutex m;std::thread t([&]{std::lock_guard<std::mutex> g(m);++n;});t.join();std::cout<<n<<"\n";}
CPP_EQUIVALENT_END */

use std::{sync::{Arc,Mutex},thread};
fn main(){let n=Arc::new(Mutex::new(0));let worker=Arc::clone(&n);let h=thread::spawn(move||*worker.lock().unwrap()+=1);h.join().unwrap();println!("{}",*n.lock().unwrap());}
