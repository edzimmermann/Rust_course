// Topic: Strings and Slices
// Exercise 09: split_once_option
// Goal: String parsing returns Option
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="key=value";auto p=s.find('=');std::cout<<s.substr(0,p)<<":"<<s.substr(p+1)<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s="key=value";let (k,v)=s.split_once('=');println!("{k}:{v}");}
