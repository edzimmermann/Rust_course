// Topic: Threads Channels and Synchronization
// Exercise 06: channel_send_result
// Goal: send can fail
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <queue>
int main(){std::queue<int> q;q.push(7);std::cout<<q.front()<<"\n";}
CPP_EQUIVALENT_END */

use std::sync::mpsc::channel;
fn main(){let (tx,rx)=channel();tx.send(7);println!("{}",rx.recv().unwrap());}
