// struct User {
//     name: String,
//     sex: bool,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = User {
//         email: String::from("woshinidie"),
//         sex: false,
//         name: String::from("nizhenniubi"),
//         sign_in_count: 99,
//     };
//     println!("{}", user1.sex);
// }

// 设置一个可变的结构体实例

// fn main() {
//     // let mut user1 = User {
//     //     email: String::from("someone@example.com"),
//     //     name: String::from("someusername123"),
//     //     sex: false,
//     //     sign_in_count: 1,
//     // };

//     // user1.email = String::from("anotheremail@example.com");
//     let s = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
//     println!("{}", s.email);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         name: username,
//         sex: true,
//         sign_in_count: 1,
//     }
// }
// // 字面量语法简写
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// 使用结构体更新语法 从其他实例中创建新的实例
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // --snip--

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@example.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };
//     // 这种情况我们可以使用user1
//     let user2 = User {
//         email: String::from("anoadasdsadadther@example.com"),
//         ..user1
//     };
//     // 这种情况 我们就相当于 把user1 交给了 user2 我们就获取不到user1了
//     let user2 = User {
//         // email: String::from("anoadasdsadadther@example.com"),
//         ..user1
//     };
//     // 请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据
//     println!("{}", user2.username);
//     println!("{}", user2.email);
//     println!("{}", user1.email);
// }

// 元祖结构体
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("{} {} {}", origin.0, origin.1, origin.2);
// }

// 没有任何字段的类单元结构体
// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
//     println!("{} ", subject);
// }

// 结构体程序实例

// fn main() {
//     let width = 30;
//     let height = 50;
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, height)
//     );
// }
// fn area(width: usize, height: usize) -> usize {
//     width * height
// }

// 程序没有把这两个参数之间的关联体现出来
// 使用元组重构
// fn main() {
//     let rect = (30, 50);
//     println!("The area of the rectangle is {} square pixels.", area(rect))
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 元组帮助我们增加了一些结构性 这样我们只需要传递一个参数
// 但是函数中的结构 却让人费解了 你不知道 这个 0 1 到底指的是什么

// 使用结构体重构 赋予更多的意义

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let react = Rectangle {
        width: 30,
        height: 60,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&react)
    );
    println!("{}", react.height);
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}