// Topic: Structs Enums and Patterns
// Solution 09: matches_macro_pattern
// Fix: Match the payload-carrying variant as Code::Err(_).

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <variant>
struct Ok{}; struct Err{int code;};
int main(){std::variant<Ok,Err> c=Err{5};std::cout<<std::holds_alternative<Err>(c)<<"\n";}
CPP_EQUIVALENT_END */

enum Code{Ok,Err(i32)}
fn main(){let c=Code::Err(5);println!("{}",matches!(c,Code::Err(_)));}
