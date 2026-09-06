// Topic: Option Result and Error Handling
// Exercise 09: return_ok_wrapper
// Goal: Returning Result success
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
std::optional<double> reciprocal(double x){if(x==0)return std::nullopt;return 1.0/x;}
int main(){std::cout<<*reciprocal(2)<<"\n";}
CPP_EQUIVALENT_END */

fn reciprocal(x:f64)->Result<f64,&'static str>{if x==0.0{return Err("zero");}1.0/x}
fn main(){println!("{}",reciprocal(2.0).unwrap());}
