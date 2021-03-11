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
    let gs = vec![4, 1];
    println!("ソート前の配列：{:?}", arr);
    shell_sort(&mut arr, gs);
    println!("ソート前の配列：{:?}", arr);
}

fn shell_sort(arr: &mut Vec<i32>, gs: Vec<usize>) {
    for g in gs {
        insertion_sort(arr, g);
    }
}

fn insertion_sort(arr: &mut Vec<i32>, g: usize) {
    for i in g..arr.len() {
        let key = arr[i];
        let mut key_index = i; // keyの値のindexを記録する
        let length = arr.len();

        // [0] [1] [2] [3] [4,0] [5,1] のような配列を作る
        for j in (0..(length / g))
            .map(|x| x * g + i % g)
            .filter(|&x| x < i)
            .rev()
        {
            if arr[j] > key {
                arr[j + g] = arr[j];
                key_index = j
            }
        }
        arr[key_index] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_num_ascending() {
        let mut arr = vec![5, 1, 4, 3, 2, 7, 9, 8, 6, 10];
        let gs = vec![4, 1];
        shell_sort(&mut arr, gs);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
