pub fn inspect<T, R, Op: FnOnce(&T) -> R>(a: &Option<T>, op: Op) -> Option<R> {
    if let Some(value) = a {
        Some(op(&value))
    } else {
        None
    }
}
