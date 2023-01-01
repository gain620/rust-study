mod r#move;
mod move2;
mod slice;

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // borrow of moved value: `s1` [E0382]
    // println!("{}, world!", s1);

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);


    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    r#move::main2();
    move2::main3();

    slice::main();
}
