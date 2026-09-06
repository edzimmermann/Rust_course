// Topic: Lifetimes
// Exercise 06: temporary_owner_in_view
// Goal: A view needs a live owner
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <string_view>
struct View{std::string_view s;};
int main(){std::string owner="abc";View v{owner};std::cout<<v.s<<"\n"<<v.s.size()<<"\n";}
CPP_EQUIVALENT_END */

struct View<'a>{s:&'a str}
fn main(){let v;{let owner=String::from("abc");v=View{s:&owner};}println!("{}",v.s);}
