// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => { //no args
        //源码展开
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
