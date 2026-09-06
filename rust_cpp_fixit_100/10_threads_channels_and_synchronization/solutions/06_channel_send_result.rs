// Topic: Threads Channels and Synchronization
// Solution 06: channel_send_result
// Fix: Handle the Result from send instead of silently ignoring a possible disconnect.

/* CPP_EQUIVALENT_BEGIN
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <queue>
int main(){std::queue<int> q;q.push(7);std::cout<<q.front()<<"\n";}
CPP_EQUIVALENT_END */

use std::sync::mpsc::channel;
fn main(){let (tx,rx)=channel();tx.send(7).unwrap();println!("{}",rx.recv().unwrap());}
