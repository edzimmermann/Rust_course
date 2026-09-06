// Topic: Smart Pointers and Interior Mutability
// Solution 08: recursive_type_box
// Fix: Insert Box indirection so the recursive enum has a finite size.

/* CPP_EQUIVALENT_BEGIN
#include <memory>
struct Node{int value;std::unique_ptr<Node> next;};
int main(){Node n{1,nullptr};}
CPP_EQUIVALENT_END */

enum List{Cons(i32,Box<List>),Nil}
fn main(){let _=List::Cons(1,Box::new(List::Nil));}
