// Topic: Collections and Iterators
// Solution 05: hashmap_entry
// Fix: Use get_mut when the map value must be modified.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <unordered_map>
int main(){std::unordered_map<std::string,int> m{{"rust",1}};++m["rust"];std::cout<<m["rust"]<<"\n";}
CPP_EQUIVALENT_END */

use std::collections::HashMap;
fn main(){let mut m=HashMap::new();m.insert("rust",1);*m.get_mut("rust").unwrap()+=1;println!("{}",m["rust"]);}
