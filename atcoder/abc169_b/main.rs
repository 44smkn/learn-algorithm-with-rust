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
    let _n = input!(u64);
    let a = input!(u64, 1);

    if !a.iter().find(|&&v| v == 0).is_none() {
        println!("0");
        return;
    }

    let mut ans = Some(1u64);
    for v in a.iter() {
        ans = ans.unwrap().checked_mul(*v);
        if ans.is_none() || ans.unwrap() > 1000000000000000000 {
            ans = None;
            break;
        }
    }
    println!("{}", ans.map_or("-1".to_string(), |v| v.to_string()));
}