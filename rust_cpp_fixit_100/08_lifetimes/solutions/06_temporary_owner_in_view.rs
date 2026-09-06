// Topic: Lifetimes
// Solution 06: temporary_owner_in_view
// Fix: Bind the owner to a local variable that outlives the view.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <string_view>
struct View{std::string_view s;};
int main(){std::string owner="abc";View v{owner};std::cout<<v.s<<"\n"<<v.s.size()<<"\n";}
CPP_EQUIVALENT_END */

struct View<'a>{s:&'a str}
fn main(){let owner=String::from("abc");let v=View{s:&owner};println!("{}",v.s);let n=v.s.len();println!("{n}");}
