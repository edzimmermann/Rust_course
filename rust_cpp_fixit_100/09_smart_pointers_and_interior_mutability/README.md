# 09. Smart Pointers and Interior Mutability

Each exercise contains one small Rust issue. The embedded C++ program is correct and shows the intended behavior.

| # | Exercise | Concept |
|---:|---|---|
| 01 | `01_rc_clone.rs` | Shared ownership with Rc |
| 02 | `02_rc_no_direct_mutation.rs` | Rc does not provide shared mutation |
| 03 | `03_refcell_borrow_mut.rs` | Interior mutability |
| 04 | `04_refcell_runtime_borrow.rs` | RefCell borrow rules are runtime checked |
| 05 | `05_box_deref.rs` | Dereferencing Box values |
| 06 | `06_weak_upgrade.rs` | Weak references may be expired |
| 07 | `07_cell_set.rs` | Cell uses get and set |
| 08 | `08_recursive_type_box.rs` | Recursive types need indirection |
| 09 | `09_try_unwrap_shared_rc.rs` | Unwrapping Rc requires unique ownership |
| 10 | `10_rc_refcell_shared_mutation.rs` | Combine Rc and RefCell intentionally |
