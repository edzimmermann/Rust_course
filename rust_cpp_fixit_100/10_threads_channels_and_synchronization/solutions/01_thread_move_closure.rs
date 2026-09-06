// Topic: Threads Channels and Synchronization
// Solution 01: thread_move_closure
// Fix: Use move so the spawned thread owns its captured data.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <thread>
int main(){std::string msg="hello";std::thread t([msg]{std::cout<<msg<<"\n";});t.join();}
CPP_EQUIVALENT_END */

use std::thread;
fn main(){let msg=String::from("hello");let h=thread::spawn(move||println!("{msg}"));h.join().unwrap();}
