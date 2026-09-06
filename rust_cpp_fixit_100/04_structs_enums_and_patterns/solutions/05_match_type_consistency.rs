// Topic: Structs Enums and Patterns
// Solution 05: match_type_consistency
// Fix: Return the same type from both match arms.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){bool ok=true;std::string value=ok?"42":"no";std::cout<<value<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let ok=true;let value=match ok{true=>"42",false=>"no"};println!("{value}");}
