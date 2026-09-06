# Instructor Key

Concise fixes for all 100 exercises. See the matching file under `solutions/` for complete corrected code.

## 01 ownership and moves

- `01_move_after_assignment.rs` — Clone the String before assigning it when both values are needed.
- `02_move_into_function.rs` — Borrow the string instead of taking ownership.
- `03_return_moved_value.rs` — Return the owned parameter directly instead of trying to use it after moving it.
- `04_copy_integer_expectation.rs` — Remove the meaningless drop of a Copy value; integers remain usable after assignment.
- `05_partial_move_struct.rs` — Clone the field if the whole struct still needs to be used.
- `06_destructure_without_move.rs` — Bind the field by reference with ref to avoid moving it out.
- `07_clone_wrong_side.rs` — Clone at the first assignment so the original remains valid.
- `08_vec_moved_by_iteration.rs` — Iterate over &v so the vector is borrowed rather than consumed.
- `09_array_of_strings_move.rs` — Borrow the indexed String instead of moving from the array.
- `10_mem_replace.rs` — Use std::mem::take to move out while leaving a valid replacement.

## 02 borrowing and references

- `01_two_mutable_borrows.rs` — Make the mutable borrows non-overlapping.
- `02_mutable_and_immutable.rs` — Perform the mutation before creating the immutable borrow.
- `03_borrow_parameter.rs` — Accept a borrowed string slice.
- `04_mut_parameter_binding.rs` — Use &mut String in the function and at the call site.
- `05_borrow_after_push.rs` — Create the element reference after the vector mutation.
- `06_split_mut_slice.rs` — Use split_at_mut to prove the two mutable references are disjoint.
- `07_borrow_scope.rs` — Stop using the immutable borrow before mutating the value.
- `08_borrowed_temporary.rs` — Bind the owned temporary to a local before borrowing it.
- `09_mutable_binding_needed.rs` — Declare the owned binding mut before taking &mut.
- `10_reborrow_mut.rs` — Explicitly reborrow with &mut *r for repeated calls.

## 03 strings and slices

- `01_string_plus_str.rs` — The + implementation expects &str on the right-hand side.
- `02_byte_index_string.rs` — Use chars() (or bytes()) rather than indexing a Rust String.
- `03_slice_boundary_utf8.rs` — Slice at a UTF-8 character boundary; é occupies two bytes.
- `04_push_string.rs` — Use push_str for a string slice; push takes one char.
- `05_str_to_string.rs` — Create an owned String from the string literal.
- `06_function_prefers_str.rs` — Accept &str so both literals and Strings can be borrowed naturally.
- `07_replace_result.rs` — Use the String returned by replace; it does not mutate in place.
- `08_trim_result.rs` — Use the &str returned by trim rather than assuming in-place mutation.
- `09_split_once_option.rs` — Handle or unwrap the Option returned by split_once.
- `10_format_not_mutating.rs` — Bind the String produced by format!.

## 04 structs enums and patterns

- `01_missing_struct_field.rs` — Initialize every struct field.
- `02_mutate_immutable_struct.rs` — Make the struct binding mutable.
- `03_enum_match_exhaustive.rs` — Add the missing enum variant to the match.
- `04_if_let_pattern.rs` — Use the tuple-variant pattern Some(value).
- `05_match_type_consistency.rs` — Return the same type from both match arms.
- `06_struct_update_move.rs` — Clone before struct update if the original must remain fully usable.
- `07_tuple_struct_field.rs` — Access tuple-struct fields with .0, .1, etc.
- `08_method_receiver_mut.rs` — Use &mut self and a mutable binding for a mutating method.
- `09_matches_macro_pattern.rs` — Match the payload-carrying variant as Code::Err(_).
- `10_while_let_pop.rs` — Match the Option value directly rather than a reference to it.

## 05 option result and error handling

- `01_question_mark_return_type.rs` — Return Result so ? can propagate the parse error, then wrap success in Ok.
- `02_option_unwrap_or_type.rs` — unwrap_or must receive the same owned type stored in the Option.
- `03_result_unwrap_or_else.rs` — Result::unwrap_or_else passes the error to its closure.
- `04_map_vs_and_then.rs` — Use and_then when the mapping function already returns an Option.
- `05_ok_or_conversion.rs` — Convert Option to Result with ok_or (or ok_or_else).
- `06_option_as_ref.rs` — Use as_ref before map to avoid consuming the Option<String>.
- `07_result_match_arms.rs` — Match both Ok and Err.
- `08_parse_type_annotation.rs` — Specify the target numeric type for parse.
- `09_return_ok_wrapper.rs` — Wrap the successful value in Ok.
- `10_transpose_option_result.rs` — Use transpose to convert Option<Result<T,E>> into Result<Option<T>,E>.

## 06 collections and iterators

- `01_collect_type.rs` — Tell collect which collection type to build.
- `02_iter_mut.rs` — Use iter_mut to obtain mutable references.
- `03_iterator_is_lazy.rs` — Consume the iterator; for_each is appropriate for side effects.
- `04_filter_reference_level.rs` — filter receives a reference to each iterator item, so dereference one extra level.
- `05_hashmap_entry.rs` — Use get_mut when the map value must be modified.
- `06_hashmap_owned_key.rs` — Clone an owned key if it is also needed after insertion.
- `07_vec_remove_index.rs` — Collection indices use usize.
- `08_enumerate_order.rs` — Destructure enumerate items as (index, item).
- `09_sum_type.rs` — Specify the accumulator/output type for sum when inference has no other clue.
- `10_sort_mutates.rs` — sort mutates the collection and returns ().

## 07 traits and generics

- `01_generic_display_bound.rs` — Add the Display bound required by println!.
- `02_derive_debug.rs` — Derive Debug for a type printed with {:?}.
- `03_missing_trait_method.rs` — Implement the required trait method.
- `04_generic_max_copy.rs` — Add PartialOrd because the function uses >.
- `05_associated_function_call.rs` — Call an associated function with Type::function syntax.
- `06_trait_default_method.rs` — Implement the required name method and inherit the default label method.
- `07_impl_trait_one_type.rs` — impl Trait must resolve to one concrete type; use a trait object for heterogeneous branches.
- `08_clone_bound.rs` — Add a Clone bound before calling clone on a generic T.
- `09_from_conversion.rs` — Use ToString/Display formatting; String does not implement From<i32>.
- `10_trait_object_dyn.rs` — Use dyn for a trait object type.

## 08 lifetimes

- `01_dangling_reference.rs` — Return owned data instead of a reference to a local String.
- `02_two_input_lifetimes.rs` — Tie both input references and the output to an explicit lifetime.
- `03_struct_reference_lifetime.rs` — Add a lifetime parameter to a struct that stores a reference.
- `04_static_misuse.rs` — Do not promise a static lifetime for data borrowed from the caller.
- `05_method_lifetime_tied_to_arg.rs` — Explicitly tie the return lifetime to input, not to &self.
- `06_temporary_owner_in_view.rs` — Bind the owner to a local variable that outlives the view.
- `07_longest_inner_scope.rs` — Ensure every possible referent outlives the returned/used reference.
- `08_lifetime_on_impl.rs` — Declare the lifetime parameter on the impl and apply it to Holder.
- `09_return_local_slice.rs` — Return an owned String when slicing a local owner.
- `10_named_lifetime_unnecessary_static.rs` — Return the same named lifetime as the borrowed input.

## 09 smart pointers and interior mutability

- `01_rc_clone.rs` — Clone the Rc handle to increment the reference count.
- `02_rc_no_direct_mutation.rs` — Use Rc::make_mut when uniquely mutating with clone-on-write semantics.
- `03_refcell_borrow_mut.rs` — Use borrow_mut to obtain a mutable RefMut guard.
- `04_refcell_runtime_borrow.rs` — End the immutable RefCell borrow before taking a mutable borrow.
- `05_box_deref.rs` — Dereference the Box to use the contained integer in arithmetic.
- `06_weak_upgrade.rs` — upgrade returns Option<Rc<T>>; handle the possibility that the value is gone.
- `07_cell_set.rs` — Use Cell::set rather than dereference assignment.
- `08_recursive_type_box.rs` — Insert Box indirection so the recursive enum has a finite size.
- `09_try_unwrap_shared_rc.rs` — Drop other strong Rc handles before try_unwrap.
- `10_rc_refcell_shared_mutation.rs` — Wrap shared data in RefCell when single-threaded shared mutation is intentional.

## 10 threads channels and synchronization

- `01_thread_move_closure.rs` — Use move so the spawned thread owns its captured data.
- `02_arc_for_shared_thread_data.rs` — Use Arc to keep shared data available in both threads.
- `03_mutex_lock_result.rs` — Handle the LockResult before dereferencing the mutex guard.
- `04_mutable_mutex_guard.rs` — Make the mutex guard binding mutable.
- `05_drop_guard_before_relock.rs` — End or drop the first guard before locking the same non-reentrant mutex again.
- `06_channel_send_result.rs` — Handle the Result from send instead of silently ignoring a possible disconnect.
- `07_drop_extra_sender.rs` — Drop all Sender handles before relying on receiver iteration to terminate.
- `08_rc_is_not_send.rs` — Rc is not Send; use Arc for thread-safe reference counting.
- `09_shared_counter.rs` — Wrap the Mutex in Arc when both the worker and caller need ownership.
- `10_atomic_ordering.rs` — Pass an explicit memory Ordering to atomic operations.
