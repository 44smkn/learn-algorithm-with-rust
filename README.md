# learn-algorithm

## 概要

主に競プロの問題を解くことでアルゴリズムへの理解を深めていくリポジトリ  
各ディレクトリは下記のような分け方

* aoj: [プログラミングコンテスト攻略のためのアルゴリズムとデータ構造](プログラミングコンテスト攻略のためのアルゴリズムとデータ構造)
* atcoder: [AtCoder](https://atcoder.jp/contests/)
* book_algorithm: [問題解決力を鍛える！アルゴリズムとデータ構造](https://bookclub.kodansha.co.jp/product?item=0000275430)

## Rustの共通テンプレート

```rust
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
```
