use std::ffi::CString;

fn main() {
    println!("Hello, world!");

    let s1 = 0965703320.to_string();
    let s2 = "hello".to_string();


    println!("s1 = {}", s1);
    println!("s2 length = {}", calStringLen(&s2));

}



fn calStringLen(s: &String) -> usize {
    return s.len()
}
