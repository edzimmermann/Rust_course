// Topic: Threads Channels and Synchronization
// Exercise 04: mutable_mutex_guard
// Goal: Mutating through a guard
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <mutex>
#include <vector>
int main(){std::vector<int> v{1};std::mutex m;{std::lock_guard<std::mutex> g(m);v.push_back(2);}}
CPP_EQUIVALENT_END */

use std::sync::Mutex;
fn main(){let m=Mutex::new(vec![1]);let g=m.lock().unwrap();g.push(2);}
