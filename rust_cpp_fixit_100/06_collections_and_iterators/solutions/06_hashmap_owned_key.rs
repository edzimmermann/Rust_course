// Topic: Collections and Iterators
// Solution 06: hashmap_owned_key
// Fix: Clone an owned key if it is also needed after insertion.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <unordered_map>
int main(){std::unordered_map<std::string,int> m;std::string key="name";m.insert({key,1});std::cout<<key<<"\n";}
CPP_EQUIVALENT_END */

use std::collections::HashMap;
fn main(){let mut m=HashMap::new();let key=String::from("name");m.insert(key.clone(),1);println!("{key}");}
