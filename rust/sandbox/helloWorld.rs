fn main() {
    let mut a: u8 = 1;
    let b: f32 = 2.34523122;
    let c = a as f32 / b;
    println!("Hello World {}, {}", a, b);

    a = 3;
    println!("Hello World {}, {:08.3},\n{:.3}", a, b, c);
    let finger = '\u{261D}';
    print!(
        "a: {0}, b: {1:08.3},\nc: {2:.3}, a: {0}\nfinger {3}\n",
        a, b, c, finger
    );
}
