// fn main() {
//     // let mut x = 6;
//     // println!("this values of x is {}", x);
//     // println!("this values of x is {}", x);
//     // const YY: i32 = 99999;
//     // println!("this values of yy is {}", YY);
//     // let x = 5;

//     // let x = x + 1;
//     // x = 8;

//     {
//         // let x = x * 2;
//         // println!("The value of x in the inner scope is: {}", x);
//     }

//     // println!("The value of x is: {}", x);
//     // let guess: u32 = "42".parse().expect("Not a number!");
//     // let res: u128 = "1231212312312".parse().expect("to large number");
//     // 整数（integer）是没有小数部分的数字。我们在第 2 章使用过一个整数类型（整型），
//     // 即 u32 类型。此类型声明表明它关联的值应该是占用 32 位空间的无符号整型（有符号整型以 i 开始，i 是英文单词 integer 的首字母
//     // ，与之相反的是 u，代表无符号 unsigned 类型）。表 3-1 显示了 Rust 中的内置的整数类型。
//     // 我们可以使用这些定义形式中的任何一个来声明整数值的类型。
//     let a: f32 = -0.01;
//     let b: i8 = -10;
//     println!("a {}", a);
// }
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let floored = 2 / 3; // Results in 0

//     // remainder
//     let remainder = 43 % 5;
// }

// 元祖类型结构赋值
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

// 类似对象形式访问 但是取出来的是值的索引
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }

// 数组的类型 + 数组的长度
// let a: [i32; 5] = [1, 2, 3, 4, 5];
// 数组的值 + 数组的长度
// let a = [3; 5]; let a = [3, 3, 3, 3, 3];
