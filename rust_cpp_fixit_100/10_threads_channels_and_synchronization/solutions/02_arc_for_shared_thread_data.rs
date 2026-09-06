// Topic: Threads Channels and Synchronization
// Solution 02: arc_for_shared_thread_data
// Fix: Use Arc to keep shared data available in both threads.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
#include <thread>
int main(){auto data=std::make_shared<std::string>("rust");std::thread t([data]{std::cout<<*data<<"\n";});t.join();std::cout<<*data<<"\n";}
CPP_EQUIVALENT_END */

use std::{sync::Arc,thread};
fn main(){let data=Arc::new(String::from("rust"));let worker=Arc::clone(&data);let h=thread::spawn(move||println!("{worker}"));h.join().unwrap();println!("{data}");}
