fn main() {
    let mut i : i32;
    let mut data= [22, 12, 13, 17, 18];
    for i in 0..data.len() {
        data[i] = floored_half(data[i]);
    }
    for i in 0..data.len() {
    println!("{}" , data[i]);
    }
}

fn floored_half(data: i32) -> i32 {
    return data / 2
}
