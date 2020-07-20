// scalar types

// integar
/* 
 * 支持的类型：
 *  length   signed   unsigned
 *  8-bit      i8        u8
 *  16-bit     i16       u16   
 *  32-bit     i32       u32     这是rust默认采用的类型
 *  64-bit     i64       u64
 *  128-bit    i128      u128
 *  arch       isize     usize   官方文档提到这种类型适合索引某些类型的集合
 * 
 * 当你知道一个变量只会存储正整数时可以使用unsigned
 * 有符号整数可以存储 -(2的n-1次方) --> (2的n-1次方-1) n代表你声明类型时选择的类型位数 对于i8而言就是 -128 --> 127
 * 无符号整数可以存储 0 --> （2的n次方 - 1）对于u8而言就是 0 --> 255
 * 
 * 字面量实例（_代表分隔符）： 
 *  Number literals       Example
 *  Decimal               98_222
 *  Hex                   0xff
 *  Octal                 0o77
 *  Binary                0b1111_0000
 *  Byte(u8 only)         b'A'
 * 
 * 当整数数值溢出时，在debug模式下会出现异常
 * 而在release版本下，溢出会被自动处理 256会变成0，257变成1。
 * 
 */

// floating
/*
 * 浮点类型分两种 f32 和 f64 默认类型 f64 
 * 
 */

// boolean
let t = true;
let f: bool = false; // with explicit type annotation


// char
/*
 * char类型使用单引号声明，和字符串是有区别的 
 * 
 */
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';


// + - * / %

// addition
let sum = 5 + 10;
// subtraction
let difference = 95.5 - 4.3;
// multiplication
let product = 4 * 30;
// division
let quotient = 56.7 / 32.2;
// remainder
let remainder = 43 % 5;


// compound types

// tuple

let tup: (i32, f64, u8) = (500, 6.4, 1); // 带类型注解的元组声明, 类型注解是可选的

let anotherTup = (500, 6.4, 1); 
let (x, y, z) = anotherTup; // 解构赋值
println!("The value of y is {}", y);

let five_hundred = tup.0; // 使用.和下标来访问元组内的值


// array

let a = [1, 2, 3, 4, 5]; // 要注意的是，rust中的数组虽然不用声明长度再赋值，但是赋值以后长度就不可以再变化
let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 通过:[type; length]这种写法规定类型和长度
let anotherArr = [3; 5]; // 这种写法看起来和上面很像，但是有一些不同，前面是初始化的值，后面是长度，也就是用5个3来初始化数组。
let firstElementInArr = anotherArr[0]; // 访问数组中的值
/*
 * rust会对提供的访问数组的下标进行检查，如果超出范围就会立即退出
 *
 */



// vector
// ...to be continued


// string
{
    let mut s = String::from("hello"); // ①
    s.push_str(", world!");
    println!("{}", s);
} // ②

/* 
 * 字符串常量是不可以修改的，因为在编译时我们就已经知道了它的具体值，所以会以硬编码的形式加入到最终的执行流
 * 所以字符串常量效率很高，这种效率就来自于不变性
 * 
 * 但是对于运行时会发生变化的值，我们没法将一些不确定大小和会变化的文本放进内存
 * 为了支持可变可拓展的字符串，我们需要在运行时而非编译时分配一些堆内存①
 * 并且需要一种可以将不再需要的内存还给分配符的方式②
 * 其他语言中可能会有gc来负责回收这部分内存
 * rust则会在变量超出作用域以后自动执行drop函数回收这部分内存
 */


 
// string slice （切片保存的是切点索引和长度）
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();                     // convert string to array
    for (i, &item) in bytes.iter().enumerate() {  // iter the array of bytes
        if item == b' ' {
            return i;
        }
    }
    s.len()
} // 这是一段获取字符串第一个词结尾index的函数
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
} // 如果在获取到index以后清除了原来字符串的内容，先前的index就没有意义了


fn main () {
    let s = String::from("hello");

    let slice = &s[0..2]; // let slice = &s[..2];
    
    let len = s.len();
    let slice2 = &s[3..len]; // let slice2 = &s[3..];

    let slice3 = &s[0..len]; // let slice3 = &s[..];
}


// 所以有了slice，我们可以重写上面的函数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// 字符串常量本身也是字符切片
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}


// array也是可以适用slice的
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3]; // type: &[i32]


// struct
// define
struct User {
    username: String,  // not &str
    email: String,
    sign_in_count: u64,
    active: bool,
}
// create instance
let user1 = User {
    email: String::from("some@example.com"),
    username: String::from("someone123"),
    active: true,
    sign_in_count: 1,
} 
// change instance field
user1.email = String::from("another@email.com");
// function return struct
// this looks tedious
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
// much better
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// create instance 
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1 // 就像js展开运算符一样，但是要注意的是仅使用没有写出的部分
};

// tuple struct
// define
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// use
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);


// unit-like struct that don't have any fields can be useful in situations in which you need to 
// implement a trait on some type but don't have any data that you want to store in the type in the type itself

// example 
// see the folder rectangle

