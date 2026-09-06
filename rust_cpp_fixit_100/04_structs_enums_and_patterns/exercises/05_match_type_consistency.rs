// Topic: Structs Enums and Patterns
// Exercise 05: match_type_consistency
// Goal: Match arms share a type
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){bool ok=true;std::string value=ok?"42":"no";std::cout<<value<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let ok=true;let value=match ok{true=>42,false=>"no"};println!("{value:?}");}
