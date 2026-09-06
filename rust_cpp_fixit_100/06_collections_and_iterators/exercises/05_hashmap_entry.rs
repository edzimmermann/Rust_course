// Topic: Collections and Iterators
// Exercise 05: hashmap_entry
// Goal: Updating a map entry
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <unordered_map>
int main(){std::unordered_map<std::string,int> m{{"rust",1}};++m["rust"];std::cout<<m["rust"]<<"\n";}
CPP_EQUIVALENT_END */

use std::collections::HashMap;
fn main(){let mut m=HashMap::new();m.insert("rust",1);*m.get("rust").unwrap()+=1;}
