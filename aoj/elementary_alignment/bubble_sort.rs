// マクロ引用元：https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!{
        n: usize,
        v: [i32; n]
    }
    let mut arr = v;
    let inv = sort(&mut arr);
    println!("ソート後の配列：{:?}", arr);
    println!("交換した回数：{}", inv);
}

fn sort(arr: &mut Vec<i32>) -> i32 {
    let mut inversions = 0;
    let n = arr.len();
    for i in 0..n {
        for j in (i + 1..n).rev() {
            if arr[j - 1] > arr[j] {
                let tmp = arr[j - 1];
                arr[j - 1] = arr[j];
                arr[j] = tmp;

                inversions += 1;
            }
        }
    }
    inversions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_num_ascending() {
        let mut arr: Vec<i32> = vec![5, 3, 2, 4, 1];
        let inv = sort(&mut arr);
        println!("交換回数：{}", inv);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
