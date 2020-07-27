// rust 会把错误分为两类，像文件未找到这种可恢复错误；以及单纯的bug导致的不可恢复错误

// 对于不可恢复的错误，panic!默认处理方式是返回堆栈，并把碰到的函数产生的数据全部清除
// 这种回退和清理会是相当的一部分工作量，你可以选择直接丢弃这部分内容，交给操作系统
// 方法是在项目的Cargo.toml文件中
[profile.release]
panic = 'abort'


// calling panic! in a simple program
fn main() {
    panic!("crash and burn");
}


// 执行时加入RUST_BACKTRACE=1
RUST_BACKTRACE=1 cargo run


// 处理可恢复性错误
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


// 处理多种可能的错误类型
use std::fs::File;
use std::io::ErrorKind;

fn main () {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            },
        },
    };
}

// after we learn more later, we can write above code in a more concise way
use std::fs::File;
use std::io::ErrorKind;

fn main () {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}


// using unwrap instead of match 
// If the Result value is the Ok variant, unwrap will return the value inside the Ok.
// If the Result is the Err variant, unwrap will call the panic! macro for us
use std::fs::File;
fn main () {
    let f = File::open("hello.txt").wrap(); 
} 



// another method: expect
// syntax is similar to unwrap but with the specific message we want
use std::fs::File;

fn main () {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}



// propagating Errors
// return the err to the calling code 
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// using ? to shorten above code 
// ? 会将错误类型转换为函数指定的错误输出类型
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


// we’re only allowed to use the ? operator in a function that 
// returns Result or Option or another type that implements std::ops::Try
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}


// 总体来说，对于可能出现错误的程序，默认返回Result是一个不错的选择
