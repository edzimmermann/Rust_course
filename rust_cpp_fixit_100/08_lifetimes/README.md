# 08. Lifetimes

Each exercise contains one small Rust issue. The embedded C++ program is correct and shows the intended behavior.

| # | Exercise | Concept |
|---:|---|---|
| 01 | `01_dangling_reference.rs` | References cannot outlive owners |
| 02 | `02_two_input_lifetimes.rs` | Lifetime elision with multiple inputs |
| 03 | `03_struct_reference_lifetime.rs` | Lifetimes on structs holding references |
| 04 | `04_static_misuse.rs` | static means program-long validity |
| 05 | `05_method_lifetime_tied_to_arg.rs` | Returning argument from a method |
| 06 | `06_temporary_owner_in_view.rs` | A view needs a live owner |
| 07 | `07_longest_inner_scope.rs` | Borrow cannot escape shorter scope |
| 08 | `08_lifetime_on_impl.rs` | Lifetime parameter on impl block |
| 09 | `09_return_local_slice.rs` | Slices borrow their source |
| 10 | `10_named_lifetime_unnecessary_static.rs` | Choose the right lifetime |
