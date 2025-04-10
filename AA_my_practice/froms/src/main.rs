use std::convert::From;//use from trait for conversion


// let a: A = From::from(b); // 从 b 构造 a
// let a: A = b.into();      // 把 b 转换成 a

#[derive(Debug)]
struct Number{
    val:i32,
}

impl From<i32> for Number{
    fn from(i32_val:i32)->Self{
        return Number{val:i32_val};
    }
}

fn main() {

    let my_str:&str = "hello";
    println!("{}",my_str);

    //from
    let my_string = String::from(my_str);
    println!("{}",my_string);

    //自定义 i32->Number
    let i32_n:i32 = 132323;
    let num_struct:Number = Number::from(i32_n);
    dbg!(&num_struct);
    println!("Number struct {:?}",num_struct);
    //默认实现了Number->i32
    // let i32_n = num_struct::into
    // 尝试移除类型标注
    // let num: Number = i32_n.into();
    let nst:i32 = num_struct.into();
    println!("我的数字是 {}", nst);


}
