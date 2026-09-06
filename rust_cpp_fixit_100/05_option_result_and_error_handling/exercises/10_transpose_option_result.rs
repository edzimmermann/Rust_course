// Topic: Option Result and Error Handling
// Exercise 10: transpose_option_result
// Goal: Option<Result> to Result<Option>
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
#include <string>
std::optional<int> maybe_number(const std::optional<std::string>& s){if(!s)return std::nullopt;return std::stoi(*s);}
int main(){std::cout<<*maybe_number(std::string("7"))<<"\n";}
CPP_EQUIVALENT_END */

fn maybe_number(s:Option<&str>)->Result<Option<i32>,std::num::ParseIntError>{s.map(|x|x.parse::<i32>())}
fn main(){println!("{:?}",maybe_number(Some("7")).unwrap());}
