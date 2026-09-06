// Topic: Lifetimes
// Exercise 10: named_lifetime_unnecessary_static
// Goal: Choose the right lifetime
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <string_view>
std::string_view echo(std::string_view s){return s;}
int main(){std::string s="echo";std::cout<<echo(s)<<"\n";}
CPP_EQUIVALENT_END */

fn echo<'a>(s:&'a str)->&'static str{s}
fn main(){let s=String::from("echo");println!("{}",echo(&s));}
