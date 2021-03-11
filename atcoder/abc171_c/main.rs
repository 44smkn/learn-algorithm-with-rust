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

// https://drken1215.hatenablog.com/entry/2020/06/21/225500
fn main() {
    let mut n = input!(u64);
    let mut mid = Vec::new();
    while n > 0 {
        n -= 1;
        let code = 'a' as u32 + (n % 26) as u32;
        let c = std::char::from_u32(code).unwrap();
        mid.push(c);
        n /= 26;
    }
    mid.reverse();
    let ans: String = mid.into_iter().collect();

    println!("{}", ans);
}