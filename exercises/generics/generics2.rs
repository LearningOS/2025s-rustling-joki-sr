// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper <T>{
    value: T,
}

// 使用泛型参数前，依然需要提前声明：impl<T>，
// 只有提前声明了，我们才能在Point<T>中使用它，
// 这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。
// 需要注意的是，这里的 Point<T> 不再是泛型声明，
// 而是一个完整的结构体类型，因为我们定义的结构体就是 Point<T> 而不再是 Point。
impl<T> Wrapper <T>{
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
