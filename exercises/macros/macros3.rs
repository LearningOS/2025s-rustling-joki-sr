// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


// #[macro_use(lazy_static)] // 或者使用 #[macro_use] 来导入所有宏.
// extern crate lazy_static;

#[macro_use]//通过作用于模块的方式让模块内的宏的作用域在模块关闭时不结束
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
