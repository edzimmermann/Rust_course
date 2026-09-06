# 10. Threads Channels and Synchronization

Each exercise contains one small Rust issue. The embedded C++ program is correct and shows the intended behavior.

| # | Exercise | Concept |
|---:|---|---|
| 01 | `01_thread_move_closure.rs` | Thread closures need owned captures |
| 02 | `02_arc_for_shared_thread_data.rs` | Share ownership across threads |
| 03 | `03_mutex_lock_result.rs` | Mutex::lock returns Result |
| 04 | `04_mutable_mutex_guard.rs` | Mutating through a guard |
| 05 | `05_drop_guard_before_relock.rs` | Avoid self-deadlock |
| 06 | `06_channel_send_result.rs` | send can fail |
| 07 | `07_drop_extra_sender.rs` | Receiver iteration ends when senders are dropped |
| 08 | `08_rc_is_not_send.rs` | Use Arc across threads |
| 09 | `09_shared_counter.rs` | Arc Mutex counter |
| 10 | `10_atomic_ordering.rs` | Atomic operations require an ordering |
