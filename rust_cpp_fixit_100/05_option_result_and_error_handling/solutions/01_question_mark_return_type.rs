// Topic: Option Result and Error Handling
// Solution 01: question_mark_return_type
// Fix: Return Result so ? can propagate the parse error, then wrap success in Ok.

/* CPP_EQUIVALENT_BEGIN
#include <charconv>
#include <iostream>
#include <optional>
#include <string_view>
std::optional<unsigned> parse_port(std::string_view s){unsigned p{};auto r=std::from_chars(s.data(),s.data()+s.size(),p);if(r.ec!=std::errc{})return std::nullopt;return p;}
int main(){std::cout<<*parse_port("8080")<<"\n";}
CPP_EQUIVALENT_END */

fn parse_port(s:&str)->Result<u16,std::num::ParseIntError>{let p:u16=s.parse()?;Ok(p)}
fn main(){println!("{}",parse_port("8080").unwrap());}
