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
    input! {
        n: usize,
        v: [i32; n]
    }
    let mut arr = v;
    println!("ソート前の配列：{:?}", arr);
    sort(&mut arr);
    println!("ソート前の配列：{:?}", arr);
}

fn sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        // iは未ソート部分列の先頭で、最初はすべて未ソートとみなすので0からスタート
        let mut minj = i;
        for j in i..arr.len() {
            if arr[j] < arr[minj] {
                minj = j;
            }
        }
        let tmp = arr[minj];
        arr[minj] = arr[i];
        arr[i] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_num_ascending() {
        let mut arr = vec![5, 6, 4, 2, 1, 3];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}
