use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    
    // 测试or_insert方法的提示和跳转
    map.entry(String::from("test")).or_insert(42);
    
    // 其他HashMap方法
    map.insert(String::from("hello"), 123);
    
    println!("{:?}", map);
} 