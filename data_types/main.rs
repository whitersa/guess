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
 * 
 * 
 * 
 * 
 * 
 */


// compound types