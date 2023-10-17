// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.
use std::fmt;

fn main() {
    // let name = "world";
    println!("Hello {}!", "world");

    println!("base 10: {:b}", 6); /* 使用了 fmt::Binary trait*/
    println!("base 8: {:o}", 10); /* 使用了 fmt::Octal trait */
    println!("base 16: {:x}", 255); /* 使用了 fmt::Lower trait */
    println!("base 16: {:X}", 255); /* 使用了 fmt::UpperHex trait*/
    println!("exp: {:e}", 10000); /* 使用了 fmt::UpperExp trait*/
    println!("exp: {:E}", 10000); /* 使用了 fmt::LowerExp trait*/

    // 等宽输出
    println!("{:0<5}", 255);
    println!("{:0>5}", 255);
    println!("{:>5}", 255);

    // 小数留位，会四舍五入
    println!("{:0>7.3}", 1.22);
    println!("{:.2}", 1.227);
    println!("{:.2}", 1.221);
    println!("{number:0>width$}", number=255, width=5);

    //只有实现了 fmt::Display trait 的类型可以使用 {}, 用户自定义的类型默认是没有实现的
    // 这里让编译器自动为结构体生成 fmt::Debug trait 使用 {:?}
    #[derive(Debug)]
    struct User {
        Id: i32,
        Name: String,
    }
    let user = User{Id:13,Name: "jack".to_string()};
    println!("{:?}", user);
    println!("{:#?}", user);

    // 为结构体实现 fmt::Display trait
    struct Person {
        Name: String,
        Age: u32,
    }
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Person: {} is {}", self.Name, self.Age)
        }
    }
    println!("{}", Person{Name: "tom".to_string(),Age: 13});

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "List: [")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", "); }
                write!(f, "{count}: {v}")?;
            }

            write!(f, "]")
        }

    }

    let v = List(vec![1,2,3]);
    println!("{}", v);

    let name = "jack";
    println!("{}",format!("hello {name}!"));

}