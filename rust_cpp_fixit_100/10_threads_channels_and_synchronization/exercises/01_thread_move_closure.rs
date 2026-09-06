// Topic: Threads Channels and Synchronization
// Exercise 01: thread_move_closure
// Goal: Thread closures need owned captures
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <thread>
int main(){std::string msg="hello";std::thread t([msg]{std::cout<<msg<<"\n";});t.join();}
CPP_EQUIVALENT_END */

use std::thread;
fn main(){let msg=String::from("hello");let h=thread::spawn(||println!("{msg}"));h.join().unwrap();}
