// Topic: Threads Channels and Synchronization
// Exercise 02: arc_for_shared_thread_data
// Goal: Share ownership across threads
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
#include <thread>
int main(){auto data=std::make_shared<std::string>("rust");std::thread t([data]{std::cout<<*data<<"\n";});t.join();std::cout<<*data<<"\n";}
CPP_EQUIVALENT_END */

use std::thread;
fn main(){let data=String::from("rust");let h=thread::spawn(move||println!("{data}"));h.join().unwrap();println!("{data}");}
