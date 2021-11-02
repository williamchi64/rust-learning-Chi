fn main() {
    section3();
}
// variable type
fn _section1() {
    //default int is i32 (-2^31~2^31-1)
    let a1 = 5;
    let a2:i32 = 5;
    assert_eq!(a1,a2);
    //u32 (0~2^32-1) is not equal to i32, even number is same
    let _b1:u32 = 5;
    //assert_eq!(a1,b1);
}
// mut and non mut variable
fn _section2() {
    let s1 = String::new();
    let mut s2 = String::new();
    //s1.push_str("s1");
    s2.push_str("s2");
    println!("{}",s1);
    println!("{}",s2);
    //s1 = String::from("Hello");
    s2 = String::from("Hi");
    println!("{}",s1);
    println!("{}",s2);
}
// shadowing of Variable
fn section3() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x: {}", x)
}