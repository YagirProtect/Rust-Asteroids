use std::sync::atomic::{AtomicU32, Ordering};

static NEXT: AtomicU32 = AtomicU32::new(1);

pub fn get_uniq_id() -> u32 {
    let uniq_id = NEXT.fetch_add(1, Ordering::Relaxed);
    return uniq_id;
}