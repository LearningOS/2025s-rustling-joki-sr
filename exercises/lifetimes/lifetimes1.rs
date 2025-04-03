// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.


// Rust 不允许悬垂引用（引用指代的对象被销毁），
// 所以编译器需要知道返回的引用来自哪一个参数
// ，并确保它不会在参数的生命周期结束后继续使用。
// 函数返回值的生命周期将与两个参数的生命周期一致
// fn 后面显式声明 'a，告诉编译器 'a 是一个 泛型生命周期参数。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
