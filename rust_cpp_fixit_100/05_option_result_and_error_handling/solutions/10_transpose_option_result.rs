// Topic: Option Result and Error Handling
// Solution 10: transpose_option_result
// Fix: Use transpose to convert Option<Result<T,E>> into Result<Option<T>,E>.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
#include <string>
std::optional<int> maybe_number(const std::optional<std::string>& s){if(!s)return std::nullopt;return std::stoi(*s);}
int main(){std::cout<<*maybe_number(std::string("7"))<<"\n";}
CPP_EQUIVALENT_END */

fn maybe_number(s:Option<&str>)->Result<Option<i32>,std::num::ParseIntError>{s.map(|x|x.parse::<i32>()).transpose()}
fn main(){println!("{:?}",maybe_number(Some("7")).unwrap());}
