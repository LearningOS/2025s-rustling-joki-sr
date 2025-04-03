// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();//将 &str 转换为 Vec<char>
    if let Some(first) = chars.first_mut(){ //获取 Vec<char> 的第一个元素的可变引用
        *first = first.to_ascii_uppercase();
    }
    chars.into_iter().collect() //将 Vec<char> 转换回 String
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // vec![]
    let mut res:Vec<String> = vec![];//必须初始化
    for v in words.iter(){
        res.push(capitalize_first(v));
    }
    res
}


// + 用于拼接 String 类型的值，但是它的底层实现要求：
// impl Add<&str> for String {
//     type Output = String;
    
//     fn add(self, rhs: &str) -> String {
//         // 这里的 `rhs` 需要是 `&str`
//     }
// }


// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // String::new()
    let mut res = "".to_string();
    for v in words.iter(){
        res = res + &(capitalize_first(v));
        // res.push_str(capitalize_first(v));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
