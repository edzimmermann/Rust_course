// Topic: Strings and Slices
// Solution 09: split_once_option
// Fix: Handle or unwrap the Option returned by split_once.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="key=value";auto p=s.find('=');std::cout<<s.substr(0,p)<<":"<<s.substr(p+1)<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s="key=value";let (k,v)=s.split_once('=').unwrap();println!("{k}:{v}");}
