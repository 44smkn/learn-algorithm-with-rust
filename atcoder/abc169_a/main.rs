macro_rules! input {
    ($ty:ty, 1) => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("read error");
        let mut vec = Vec::new();
        for e in s.trim().split_whitespace() {
            vec.push(e.parse::<$ty>().unwrap());
        }
        vec
    }};

    ($ty:ty, $row:expr) => {{
        let mut vec = Vec::new();
        for _ in 0..$row {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).expect("read error");
            let mut rvec = Vec::new();
            for e in s.trim().split_whitespace() {
                rvec.push(e.parse::<$ty>().unwrap());
            }
            vec.push(rvec)
        }
        vec
    }};

    ($ty: ty) => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("read error");
        s.trim().parse::<$ty>().unwrap()
    }};
}


fn main() {
    let input = input!(i32, 1);
    let a = input[0];
    let b = input[1];

    println!("{}", a*b)
}