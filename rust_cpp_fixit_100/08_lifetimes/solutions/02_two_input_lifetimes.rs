// Topic: Lifetimes
// Solution 02: two_input_lifetimes
// Fix: Tie both input references and the output to an explicit lifetime.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string_view>
std::string_view choose(std::string_view a,std::string_view b){return a.size()>=b.size()?a:b;}
int main(){std::cout<<choose("rust","c++")<<"\n";}
CPP_EQUIVALENT_END */

fn choose<'a>(a:&'a str,b:&'a str)->&'a str{if a.len()>=b.len(){a}else{b}}
fn main(){println!("{}",choose("rust","c++"));}
