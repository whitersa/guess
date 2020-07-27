// rust std library provides some useful data structure
// you can see this folder as additions to the data types

// the data these collections point to is stored on the heap(堆)
// 也就是说这部分数据编译时无需知晓，在运行时可以自由扩展收缩

// vector 向量 存储一系列相邻的值
// string 字符串 data_types 已经说了一些
// hash map 哈希图 用特定的键关联一个值，是图的一种特殊表现


// vector
// vector 只能存储相同类型的值
// create
let v: Vec<i32> = Vec::new(); // 因为我们没有插入任何值，所以加了类型标识
let v = vec![1, 2, 3]; // 更常见的代码是我们会在创建时初始化，这时候我们可以用vec!来简化写法

// update
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

// destroy
// 和其他复杂类型一样，向量超出作用域会被drop，向量保存的值也会一起被drop掉

// read
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];           // through & and [], we get a reference
println!("The third element is {}", third);

match v.get(2) {                   // through get, we get an Option<&T>
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// 对于上面两种获取值的方式，如果想要在访问超出范围的值时得到反馈，应该选择[]
// 而get方法会返回None，适合处理你知道正常情况下也会出现的超界问题


// 之前提到的可变引用和不可变引用也适用于vector
// 下面的代码就会报错，因为同时有可变和不可变引用
// 因为扩大向量可能会分配新的空间然后逐一复制过去
// 旧的引用地址可能被释放从而无法引用
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);
println!("The first element is: {}", first);


// iterate
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}


// 因为向量只能存储相同类型，如果我们想要存储不同数据，就要借助枚举
// 如果你还不清楚具体会有什么类型，后面会提到trait对象
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("Blue")),
    SpreadsheetCell::Float(10.12),
];



// string
// 向量的很多方法可以适用String
let mut s = String::new();


let data = "initial contents";
let s = data.to_string();


let s = "initial contents".to_string(); 


let s = String::from("initial contents");


let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");

// modify
let mut s = String::from("foo");
s.push_str("bar"); // push_str不会获取所有权

let mut s1 = String::from("lo");
s1.push('l');

let s2 = String::from("Hello, ");
let s3 = String::from("world!");
let s4 = s2 + &s3;  // s2 is moved to here
                    // 不能同为引用值

// 下面这样很繁琐
let s5 = String::from("tic");
let s6 = String::from("tac");
let s7 = String::from("toe");
let s8 = s5 + "-" + &s6 + "-" +&s7;

// 简写
let s9 = format!("{}-{}-{}", s1, s2, s3); // much better and not take anyone's ownership


// rust是不支持按索引也就是index查找String的，原理如下
// utf-8编码，保存的是编码值，返回编码值并没有意义，干脆不返回，直接编译不通过
// If we look at the Hindi word “नमस्ते” written in the Devanagari script, 
// it is stored as a vector of u8 values that looks like this:
// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
// 224, 165, 135]
// That’s 18 bytes and is how computers ultimately store this data. If we 
// look at them as Unicode scalar values, which are what Rust’s char type is, 
// those bytes look like this:
// ['न', 'म', 'स', '्', 'त', 'े']
// There are six char values here, but the fourth and sixth are not letters: 
// they’re diacritics that don’t make sense on their own. Finally, if we 
// look at them as grapheme clusters, we’d get what a person would call the 
// four letters that make up the Hindi word:
// ["न", "म", "स्", "ते"]

// 总而言之，按索引访问期待的时间复杂度是O(1)，面对如此多的字符类型，然后再决定有效长度，是不现实的。
let hello = String::from("Hola"); // length 4
let hello = String::from("Здравствуйте"); // length 12 ? 24!


// 那么应该如何遍历字符串呢 
for c in "नमस्ते".char() {
    println!("{}", c);
}
// and we will get 
// न
// म
// स
//  ्
// त
//  े

for b in "नमस्ते".bytes() {
    println!("{}", b);
}
// 224
// 164
// ...
// 165
// 135



// hashmap
// The type HashMap<K, V> stores a mapping of keys of type K to values of type V
// Many programming languages support this kind of data structure, but they often use 
// a different name, such as hash, map, object, hash table, dictionary, or associative array

use std::collections::HashMap;

let mut scores = HashMap::new();
// all of the keys must have the same type, and all of the values must have the same type
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Creating a hash map from a list of teams and a list of scores
use std::collections::HashMap;
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> = 
    teams.into_iter().zip(initial_scores.into_iter()).collect();


// the values will be moved and the hash map will be the owner of those values
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);


// Accessing Values in a Hash Map
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name); // result will be Some(&10), because get returns an Option<&V>
                                    // if no value for that key, get will return None


// for loop 
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value); // Yellow: 50
                                    // Blue: 10
}



// Updating a Hash Map
// You could replace the old value with the new value, completely disregarding the old value. 
// You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value.
// Or you could combine the old value and the new value.

// Overwriting a Value
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);


// Only Inserting a Value If the Key Has No Value
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);


// Updating a Value Based on the Old Value
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0); // return a mutable reference to the value (&mut V)
    *count += 1;
}

println!("{:?}", map);


// Hashing Functions
// rust默认采用 “cryptographically strong”哈希函数，虽然性能稍差，但是换来的安全性还是蛮高的
// 如果认为默认哈希函数的性能太差，你可以在crates.io中找到一些其他人分享的哈希算法