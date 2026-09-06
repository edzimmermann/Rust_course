// Topic: Lifetimes
// Exercise 02: two_input_lifetimes
// Goal: Lifetime elision with multiple inputs
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string_view>
std::string_view choose(std::string_view a,std::string_view b){return a.size()>=b.size()?a:b;}
int main(){std::cout<<choose("rust","c++")<<"\n";}
CPP_EQUIVALENT_END */

fn choose(a:&str,b:&str)->&str{if a.len()>=b.len(){a}else{b}}
fn main(){println!("{}",choose("rust","c++"));}
