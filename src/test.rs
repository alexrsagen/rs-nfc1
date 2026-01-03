use std::thread;
use crate::*;

#[test]
fn context_new_drop() {
	let context = std::hint::black_box(Context::new());
	assert!(context.is_ok());
	std::mem::drop(context);
}

#[test]
fn parallel_context_new_drop() {
	let thread_count = thread::available_parallelism();
	assert!(thread_count.is_ok());
	let thread_count = thread_count.unwrap().get();
    let mut handles = Vec::with_capacity(thread_count);
	for _ in 0..thread_count {
		handles.push(thread::spawn(move || {
			for _ in 0..1024 {
				context_new_drop();
			}
		}));
	}
    for handle in handles {
		assert!(handle.join().is_ok());
	}
}