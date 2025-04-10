// fn main(){

    
//     let s = "haha";
//     println!("{}",s);
//     let s = s.to_string();
//     println!("{}",s);
    
//     // 我想显式地调用 String 对 AsRef<str> 的那个 as_ref() 方法
//     // 这种 完全限定语法（fully qualified syntax）
//     let r = <String as AsRef<str>>::as_ref(&s);
//     println!("r:{}",r)

// }


trait ShortDescription{
    fn describe(&self)->String;
}

trait LongDescription{
    fn describe(&self)->String;
}

struct Book{
    title:String,
    author:String,
}

impl  ShortDescription for Book{
    fn describe(&self)->String{
        // return self.title;
        format!("《{}》",self.title)
    }
}

impl LongDescription for Book{
    fn describe(&self)->String{
        format!("《{}》 by {}",self.title,self.author)
    }
}

fn main(){
    let b = Book {
        title: "Rust 秘笈".to_string(),
        author: "铁锈侠".to_string(),
    };

    // b.describe(); //error
    LongDescription::describe(&b);
    ShortDescription::describe(&b);
    
    //Book that impl Short..., call describe()
    let shortmsg = <Book as ShortDescription>::describe(&b);
    let longmsg = <Book as LongDescription>::describe(&b);
    println!("short:{}, long:{}",shortmsg,longmsg);

}
