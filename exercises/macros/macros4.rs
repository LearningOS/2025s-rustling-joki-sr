// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// literal 用于字面常量
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)
// vis (可见性描述符)

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    //$参数：指示符（类型）
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
