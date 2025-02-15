//! Reactive utilities for dealing with lists and iterables.

use std::hash::Hash;
use std::mem;
use std::rc::Rc;

use ahash::AHashMap;
use husky_signal::Signalable;

use crate::*;

/// Function that maps a `Vec` to another `Vec` via a map function. The mapped `Vec` is lazy
/// computed, meaning that it's value will only be updated when requested. Modifications to the
/// input `Vec` are diffed using keys to prevent recomputing values that have not changed.
///
/// This function is the underlying utility behind `Keyed`.
///
/// # Params
/// * `list` - The list to be mapped. The list must be a [`ReadSignal`] (obtained from a [`Signal`])
///   and therefore reactive.
/// * `map_fn` - A closure that maps from the input type to the output type.
/// * `key_fn` - A closure that returns an _unique_ key to each entry.
///
///  _Credits: Based on TypeScript implementation in <https://github.com/solidjs/solid>_
pub fn map_keyed<'a, T, K, U>(
    cx: Scope<'a>,
    list: &'a ReadSignal<Vec<T>>,
    map_fn: impl for<'child_lifetime> Fn(BoundedScope<'child_lifetime, 'a>, T) -> U + 'a,
    key_fn: impl Fn(&T) -> K + 'a,
) -> &'a ReadSignal<Vec<U>>
where
    T: Eq + Clone + 'a,
    K: Eq + Hash,
    U: Clone + 'a + Signalable,
{
    // Previous state used for diffing.
    let mut items = Rc::new(Vec::new());
    let mut mapped: Vec<U> = Vec::new();
    let mut disposers: Vec<Option<ScopeDisposer<'a>>> = Vec::new();

    let signal = create_signal(cx, Vec::new());

    // Diff and update signal each time list is updated.
    effect!(cx, move || {
        let new_items = list.get();
        if new_items.is_empty() {
            // Fast path for removing all items.
            for dis in mem::take(&mut disposers) {
                unsafe {
                    dis.unwrap().dispose();
                }
            }
            mapped = Vec::new();
        } else if items.is_empty() {
            // Fast path for new create.
            // TODO: do not clone T
            for new_item in new_items.iter().cloned() {
                let mut tmp = None;
                let new_disposer = create_child_scope(cx, |cx| {
                    // SAFETY: f takes the same parameter as the argument to create_child_scope.
                    tmp = Some(map_fn(unsafe { mem::transmute(cx) }, new_item));
                });
                mapped.push(tmp.unwrap());
                disposers.push(Some(new_disposer));
            }
        } else {
            debug_assert!(
                !new_items.is_empty() && !items.is_empty(),
                "new_items.is_empty() and items.is_empty() are special cased"
            );

            let mut temp = vec![None; new_items.len()];
            let mut temp_disposers = {
                let mut tmp = Vec::with_capacity(new_items.len());
                for _ in 0..new_items.len() {
                    tmp.push(None);
                }
                tmp
            };

            // Skip common prefix.
            let min_len = usize::min(items.len(), new_items.len());
            let start = items
                .iter()
                .zip(new_items.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(min_len);
            debug_assert!(
                (items.get(start).is_none() && new_items.get(start).is_none())
                    || (items.get(start) != new_items.get(start)),
                "start is the first index where items[start] != new_items[start]"
            );

            // Skip common suffix.
            let mut end = items.len();
            let mut new_end = new_items.len();
            #[allow(clippy::suspicious_operation_groupings)]
            // FIXME: make code clearer so that clippy won't complain
            while end > start && new_end > start && items[end - 1] == new_items[new_end - 1] {
                end -= 1;
                new_end -= 1;
                temp[new_end] = Some(mapped[end].clone());
                temp_disposers[new_end] = disposers[end].take();
            }
            debug_assert!(
                    if end != 0 && new_end != 0 {
                        (end == items.len() && new_end == new_items.len())
                            || (items[end - 1] != new_items[new_end - 1])
                    } else {
                        true
                    },
                    "end and new_end are the last indexes where items[end - 1] != new_items[new_end - 1]"
                );

            // 0) Prepare a map of indices in newItems. Scan backwards so we encounter them in
            // natural order.
            let mut new_indices = AHashMap::with_capacity(new_end - start);

            // Indexes for new_indices_next are shifted by start because values at 0..start are
            // always None.
            let mut new_indices_next = vec![None; new_end - start];
            for j in (start..new_end).rev() {
                let item = &new_items[j];
                let i = new_indices.get(&key_fn(item));
                new_indices_next[j - start] = i.copied();
                new_indices.insert(key_fn(item), j);
            }

            // 1) Step through old items and see if they can be found in new set; if so, mark
            // them as moved.
            for i in start..end {
                let item = &items[i];
                if let Some(j) = new_indices.get(&key_fn(item)).copied() {
                    // Moved. j is index of item in new_items.
                    temp[j] = Some(mapped[i].clone());
                    temp_disposers[j] = disposers[i].take();
                    new_indices_next[j - start].and_then(|j| new_indices.insert(key_fn(item), j));
                } else {
                    // Create new.
                    unsafe {
                        disposers[i].take().unwrap().dispose();
                    }
                }
            }

            // 2) Set all the new values, pulling from the moved array if copied, otherwise
            // entering the new value.
            for j in start..new_items.len() {
                if matches!(temp.get(j), Some(Some(_))) {
                    // Pull from moved array.
                    if j >= mapped.len() {
                        debug_assert_eq!(mapped.len(), j);
                        mapped.push(temp[j].clone().unwrap());
                        disposers.push(temp_disposers[j].take());
                    } else {
                        mapped[j] = temp[j].clone().unwrap();
                        disposers[j] = temp_disposers[j].take();
                    }
                } else {
                    // Create new value.
                    let mut tmp = None;
                    let new_item = new_items[j].clone();
                    let new_disposer = create_child_scope(cx, |cx| {
                        // SAFETY: f takes the same parameter as the argument to create_child_scope.
                        tmp = Some(map_fn(unsafe { mem::transmute(cx) }, new_item));
                    });
                    if mapped.len() > j {
                        mapped[j] = tmp.unwrap();
                        disposers[j] = Some(new_disposer);
                    } else {
                        mapped.push(tmp.unwrap());
                        disposers.push(Some(new_disposer));
                    }
                }
            }
        }

        // 3) In case the new set is shorter than the old, set the length of the mapped array.
        mapped.truncate(new_items.len());
        disposers.truncate(new_items.len());

        // 4) Save a copy of the mapped items for the next update.
        items = Rc::clone(&new_items);
        debug_assert!([items.len(), mapped.len(), disposers.len()]
            .iter()
            .all(|l| *l == new_items.len()));

        // 5) Update signal to trigger updates.
        signal.set(mapped.clone());
    });

    signal
}

/// Function that maps a `Vec` to another `Vec` via a map function. The mapped `Vec` is lazy
/// computed, meaning that it's value will only be updated when requested. Modifications to the
/// input `Vec` are diffed by index to prevent recomputing values that have not changed.
///
/// Generally, it is preferred to use [`map_keyed`] instead when a key function
/// is available.
///
/// This function is the underlying utility behind `Indexed`.
///
/// # Params
/// * `list` - The list to be mapped. The list must be a [`ReadSignal`] (obtained from a [`Signal`])
///   and therefore reactive.
/// * `map_fn` - A closure that maps from the input type to the output type.
pub fn map_indexed<'a, T, U>(
    cx: Scope<'a>,
    list: &'a ReadSignal<Vec<T>>,
    map_fn: impl for<'child_lifetime> Fn(BoundedScope<'child_lifetime, 'a>, T) -> U + 'a,
) -> &'a ReadSignal<Vec<U>>
where
    T: PartialEq + Clone,
    U: Clone + 'a + Signalable,
{
    // Previous state used for diffing.
    let mut items = Rc::new(Vec::new());
    let mut mapped = Vec::new();
    let mut disposers: Vec<ScopeDisposer<'a>> = Vec::new();

    let signal = create_signal(cx, Vec::new());

    // Diff and update signal each time list is updated.
    effect!(cx, move || {
        let new_items = list.get();

        if new_items.is_empty() {
            // Fast path for removing all items.
            for dis in mem::take(&mut disposers) {
                unsafe {
                    dis.dispose();
                }
            }
            items = Rc::new(Vec::new());
            mapped = Vec::new();
        } else {
            // Pre-allocate space needed
            if new_items.len() > items.len() {
                let new_count = new_items.len() - items.len();
                mapped.reserve(new_count);
                disposers.reserve(new_count);
            }

            for (i, new_item) in new_items.iter().enumerate() {
                let new_item = new_item.clone();
                let item = items.get(i);
                // We lift the equality out of the else if branch to satisfy borrow checker.
                let eqs = item != Some(&new_item);

                let mut tmp = None;
                if item.is_none() || eqs {
                    let new_disposer = create_child_scope(cx, |cx| {
                        // SAFETY: f takes the same parameter as the argument to
                        // create_child_scope(cx, _).
                        tmp = Some(map_fn(unsafe { mem::transmute(cx) }, new_item));
                    });
                    if item.is_none() {
                        // SAFETY: tmp is written in create_child_scope.
                        mapped.push(tmp.unwrap());
                        disposers.push(new_disposer);
                    } else if eqs {
                        // SAFETY: tmp is written in create_child_scope.
                        mapped[i] = tmp.unwrap();
                        let prev = mem::replace(&mut disposers[i], new_disposer);
                        unsafe {
                            prev.dispose();
                        }
                    }
                }
            }

            if new_items.len() < items.len() {
                for _i in new_items.len()..items.len() {
                    unsafe {
                        disposers.pop().unwrap().dispose();
                    }
                }
            }

            // In case the new set is shorter than the old, set the length of the mapped array.
            mapped.truncate(new_items.len());

            // save a copy of the mapped items for the next update.
            items = Rc::clone(&new_items);
            debug_assert!([items.len(), mapped.len(), disposers.len()]
                .iter()
                .all(|l| *l == new_items.len()));
        }

        // Update signal to trigger updates.
        signal.set(mapped.clone());
    });

    signal
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn keyed() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let mapped = map_keyed(cx, a, |_, x| x * 2, |x| *x);
            assert_eq!(*mapped.get(), vec![2, 4, 6]);

            a.set(vec![1, 2, 3, 4]);
            assert_eq!(*mapped.get(), vec![2, 4, 6, 8]);

            a.set(vec![2, 2, 3, 4]);
            assert_eq!(*mapped.get(), vec![4, 4, 6, 8]);
        });
    }

    #[test]
    fn keyed_recompute_everything() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let mapped = map_keyed(cx, a, |_, x| x * 2, |x| *x);
            assert_eq!(*mapped.get(), vec![2, 4, 6]);

            a.set(vec![4, 5, 6]);
            assert_eq!(*mapped.get(), vec![8, 10, 12]);
        });
    }

    /// Test fast path for clearing Vec.
    #[test]
    fn keyed_clear() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let mapped = map_keyed(cx, a, |_, x| x * 2, |x| *x);

            a.set(Vec::new());
            assert_eq!(*mapped.get(), Vec::<i32>::new());
        });
    }

    /// Test that using [`Scope::map_keyed`] will reuse previous computations.
    #[test]
    fn keyed_use_previous_computation() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let counter = Rc::new(Cell::new(0));
            let mapped = map_keyed(
                cx,
                a,
                {
                    let counter = Rc::clone(&counter);
                    move |_, _| {
                        counter.set(counter.get() + 1);
                        counter.get()
                    }
                },
                |x| *x,
            );
            assert_eq!(*mapped.get(), vec![1, 2, 3]);

            a.set(vec![1, 2]);
            assert_eq!(*mapped.get(), vec![1, 2]);

            a.set(vec![1, 2, 4]);
            assert_eq!(*mapped.get(), vec![1, 2, 4]);

            a.set(vec![1, 2, 3, 4]);
            assert_eq!(*mapped.get(), vec![1, 2, 5, 4]);
        });
    }

    #[test]
    fn keyed_call_cleanup_on_remove() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let counter = Rc::new(Cell::new(0));
            let _mapped = map_keyed(
                cx,
                a,
                {
                    let counter = Rc::clone(&counter);
                    move |cx, _| {
                        let counter = Rc::clone(&counter);
                        on_cleanup(cx, move || {
                            counter.set(counter.get() + 1);
                        });
                    }
                },
                |x| *x,
            );
            assert_eq!(counter.get(), 0, "no cleanup yet");

            a.set(vec![1, 2]);
            assert_eq!(counter.get(), 1);

            a.set(vec![1, 2, 3]);
            assert_eq!(counter.get(), 1);

            a.set(vec![1, 3]);
            assert_eq!(counter.get(), 2);
        });
    }

    #[test]
    fn keyed_call_cleanup_on_remove_all() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let counter = Rc::new(Cell::new(0));
            let _mapped = map_keyed(
                cx,
                a,
                {
                    let counter = Rc::clone(&counter);
                    move |cx, _| {
                        let counter = Rc::clone(&counter);
                        on_cleanup(cx, move || {
                            counter.set(counter.get() + 1);
                        })
                    }
                },
                |x| *x,
            );
            assert_eq!(counter.get(), 0, "no cleanup yet");

            a.set(vec![]);
            assert_eq!(counter.get(), 3);
        });
    }

    #[test]
    fn indexed() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let mapped = map_indexed(cx, a, |_, x| x * 2);
            assert_eq!(*mapped.get(), vec![2, 4, 6]);

            a.set(vec![1, 2, 3, 4]);
            assert_eq!(*mapped.get(), vec![2, 4, 6, 8]);

            a.set(vec![2, 2, 3, 4]);
            assert_eq!(*mapped.get(), vec![4, 4, 6, 8]);
        });
    }

    /// Test fast path for clearing Vec.
    #[test]
    fn indexed_clear() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let mapped = map_indexed(cx, a, |_, x| x * 2);

            a.set(Vec::new());
            assert_eq!(*mapped.get(), Vec::<i32>::new());
        });
    }

    /// Test that result of mapped function can be listened to.
    #[test]
    fn indexed_react() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let mapped = map_indexed(cx, a, |_, x| x * 2);

            let counter = create_signal(cx, 0);
            effect!(cx, || {
                counter.set(*counter.get_untracked() + 1);
                mapped.track();
            });

            assert_eq!(*counter.get(), 1);
            a.set(vec![1, 2, 3, 4]);
            assert_eq!(*counter.get(), 2);
        });
    }

    /// Test that using [`map_indexed`] will reuse previous computations.
    #[test]
    fn indexed_use_previous_computation() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let counter = Rc::new(Cell::new(0));
            let mapped = map_indexed(cx, a, {
                let counter = Rc::clone(&counter);
                move |_, _| {
                    counter.set(counter.get() + 1);
                    counter.get()
                }
            });
            assert_eq!(*mapped.get(), vec![1, 2, 3]);

            a.set(vec![1, 2]);
            assert_eq!(*mapped.get(), vec![1, 2]);

            a.set(vec![1, 2, 4]);
            assert_eq!(*mapped.get(), vec![1, 2, 4]);

            a.set(vec![1, 3, 4]);
            assert_eq!(*mapped.get(), vec![1, 5, 4]);
        });
    }

    #[test]
    fn indexed_call_cleanup_on_remove() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let counter = Rc::new(Cell::new(0));
            let _mapped = map_indexed(cx, a, {
                let counter = Rc::clone(&counter);
                move |cx, _| {
                    let counter = Rc::clone(&counter);
                    on_cleanup(cx, move || {
                        counter.set(counter.get() + 1);
                    });
                }
            });
            assert_eq!(counter.get(), 0, "no cleanup yet");

            a.set(vec![1, 2]);
            assert_eq!(counter.get(), 1);

            a.set(vec![1, 2, 3]);
            assert_eq!(counter.get(), 1);

            a.set(vec![1, 3]);
            assert_eq!(counter.get(), 3);
        });
    }

    #[test]
    fn indexed_call_cleanup_on_remove_all() {
        create_scope_immediate(|cx| {
            let a = create_signal(cx, vec![1, 2, 3]);
            let counter = Rc::new(Cell::new(0));
            let _mapped = map_indexed(cx, a, {
                let counter = Rc::clone(&counter);
                move |cx, _| {
                    let counter = Rc::clone(&counter);
                    on_cleanup(cx, move || {
                        counter.set(counter.get() + 1);
                    })
                }
            });
            assert_eq!(counter.get(), 0, "no cleanup yet");

            a.set(vec![]);
            assert_eq!(counter.get(), 3);
        });
    }
}
