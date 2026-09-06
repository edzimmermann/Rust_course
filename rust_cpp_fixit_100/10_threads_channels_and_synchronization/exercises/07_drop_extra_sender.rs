// Topic: Threads Channels and Synchronization
// Exercise 07: drop_extra_sender
// Goal: Receiver iteration ends when senders are dropped
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <queue>
int main(){std::queue<int> q;q.push(1);while(!q.empty()){std::cout<<q.front()<<"\n";q.pop();}}
CPP_EQUIVALENT_END */

use std::sync::mpsc::channel;
fn main(){let (tx,rx)=channel();let tx2=tx.clone();tx.send(1).unwrap();drop(tx);for x in rx{println!("{x}");}drop(tx2);}
