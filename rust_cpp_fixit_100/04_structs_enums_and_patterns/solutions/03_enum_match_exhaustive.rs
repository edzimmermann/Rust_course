// Topic: Structs Enums and Patterns
// Solution 03: enum_match_exhaustive
// Fix: Add the missing enum variant to the match.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string_view>
enum class State{Idle,Busy,Done};
std::string_view text(State s){switch(s){case State::Idle:return "idle";case State::Busy:return "busy";case State::Done:return "done";}return {};}
int main(){std::cout<<text(State::Done)<<"\n";}
CPP_EQUIVALENT_END */

enum State{Idle,Busy,Done}
fn text(s:State)->&'static str{match s{State::Idle=>"idle",State::Busy=>"busy",State::Done=>"done"}}
fn main(){println!("{}",text(State::Done));}
