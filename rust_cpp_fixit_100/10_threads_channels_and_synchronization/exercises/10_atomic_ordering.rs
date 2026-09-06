// Topic: Threads Channels and Synchronization
// Exercise 10: atomic_ordering
// Goal: Atomic operations require an ordering
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <atomic>
#include <iostream>
int main(){std::atomic<unsigned> n{0};n.fetch_add(1,std::memory_order_relaxed);std::cout<<n.load(std::memory_order_relaxed)<<"\n";}
CPP_EQUIVALENT_END */

use std::sync::atomic::AtomicUsize;
fn main(){let n=AtomicUsize::new(0);n.fetch_add(1);println!("{}",n.load());}
