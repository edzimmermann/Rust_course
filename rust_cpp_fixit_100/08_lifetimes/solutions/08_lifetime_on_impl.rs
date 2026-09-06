// Topic: Lifetimes
// Solution 08: lifetime_on_impl
// Fix: Declare the lifetime parameter on the impl and apply it to Holder.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string_view>
struct Holder{std::string_view s;std::string_view get()const{return s;}};
int main(){Holder h{"ok"};std::cout<<h.get()<<"\n";}
CPP_EQUIVALENT_END */

struct Holder<'a>{s:&'a str}
impl<'a> Holder<'a>{fn get(&self)->&str{self.s}}
fn main(){let h=Holder{s:"ok"};println!("{}",h.get());}
