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
        v: [i32; n],
    }
    let mut arr = v;
    sort(&mut arr);
    println!("ソート後の配列：{:?}", arr)
}

fn sort(arr: &mut Vec<i32>) {
    // iは未ソート部分列の先頭インデックス
    for i in 1..arr.len() {
        let key = arr[i];
        let mut key_index = 0;
        // jはソート済み部分列からkeyを挿入するためのループ変数。i-1から0までをループする。
        for j in (0..i).rev() {
            if arr[j] < key {
                // keyの方が大きい場合には、今のjのインデックスの右隣となるため+1している
                key_index = j + 1;
                break;
            }
            arr[j + 1] = arr[j];
        }
        // keyがソート済配列のどの要素よりも小さかった場合には、breakに入らず、インデックスが0となり、最初の要素にkeyが入る
        arr[key_index] = key;
    }
}
