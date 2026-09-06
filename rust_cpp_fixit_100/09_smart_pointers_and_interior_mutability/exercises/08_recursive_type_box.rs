// Topic: Smart Pointers and Interior Mutability
// Exercise 08: recursive_type_box
// Goal: Recursive types need indirection
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <memory>
struct Node{int value;std::unique_ptr<Node> next;};
int main(){Node n{1,nullptr};}
CPP_EQUIVALENT_END */

enum List{Cons(i32,List),Nil}
fn main(){let _=List::Cons(1,List::Nil);}
