// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]//#[allow(unused_variables, unused_assignments)]
// 作用：忽略 Clippy 的 let_unit_value lint 警告。
// 适用情况：当代码中存在 let _ = expr(); 且 expr() 返回 ()（即 unit 类型）时，Clippy 会建议去掉这种无意义的绑定。
#[allow(clippy::let_unit_value)]
// 作用：忽略 "无意义的路径语句" 警告。
// 适用情况：当代码中直接写了一个路径（如 some::path;）但没有任何作用时，Rust 会警告。
#[allow(path_statements)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(itm) = my_option{
        // my_option.unwrap();
        my_option;
    }

    let my_arr = &[
        -1, -2, -3,//lost comma
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
