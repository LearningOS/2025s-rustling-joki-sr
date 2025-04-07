// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.
// arc1.rs
//
// 在这个练习中，我们有一个包含 u32 类型数值的 Vec，名为 "numbers"，
// 其值范围从 0 到 99 —— [ 0, 1, 2, ..., 98, 99 ]。
// 我们需要在 8 个不同的线程中同时使用这组数字。
// 每个线程将计算以特定偏移量开始的每第 8 个数字的总和。
//
// 第一个线程（偏移量 0）将计算 0, 8, 16, ... 的总和
// 第二个线程（偏移量 1）将计算 1, 9, 17, ... 的总和
// 第三个线程（偏移量 2）将计算 2, 10, 18, ... 的总和
// ...
// 第八个线程（偏移量 7）将计算 7, 15, 23, ... 的总和
//
// 因为我们使用了多线程，所以我们的值需要是线程安全的。
// 因此，我们使用了 Arc（原子引用计数）。
// 我们需要在两处 TODO 注释处进行修改。
//
// 通过在第一处 TODO 注释处为 `shared_numbers` 赋值，
// 并在第二处 TODO 注释处为 `child_numbers` 创建初始绑定，
// 使这段代码能够编译通过。
// 注意不要创建 `numbers` Vec 的任何副本！
//
// 执行 `rustlings hint arc1` 或使用 `hint` watch 子命令获取提示。

// 我尚未完成

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::new(shared_numbers.clone());
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
