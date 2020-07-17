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

