use autocxx::include_cpp;

include_cpp! {
    #include "hello.h"
    safety!(unsafe)
    generate!("greet")
    generate!("add")
}

fn main() {
    let name: String = "Taro".to_string();
    ffi::greet(&name);

    let a: f32 = 10.0;
    let b: f32 = 0.4;

    println!("{}", ffi::add(a, b));
}
