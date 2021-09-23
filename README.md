# Rust プログラミング入門

[Rust プログラミング入門](https://www.udemy.com/course/rust-os-system/)

# 個人用メモ

## メモリ

- 静的領域

  静的なデータを格納する領域。  
  `const`などの定数をモジュール内のグローバルスコープで宣言すると静的領域で確保される。

- スタック

  モジュール内で宣言された変数などがデフォルトで格納される領域。  
  スタック領域で格納した変数はスコープを抜けると、メモリから解放(drop)される。

- ヒープ

  動的に確保されるメモリ。  
  `String`型や`Vector`型、`Boxポインタ`の値などがヒープ領域で格納される。  
  ヒープ領域を使用する変数には**所有権**が与えられ、**所有権**を所持している変数に対して参照やヒープメモリの解放が可能である。

## 所有権とは

- 所有権は 1 つの変数のみが所持し、その変数からのみ読み書きが行えるというルール

- 所有権システムが無い場合、メモリの二重解放エラーが引き起こされる可能性がある

  - ある変数 x と y が同じヒープ領域を参照している変数だとする(所有権を x, y ともに所持しているとする)

  - x が参照している値をヒープ領域から解放したとする

  - このとき、y は解放されたヒープ領域のメモリを参照しつづけることになる

  - その結果、意図しない値で読み込んだり、書き換えが行ってしまう可能性がある

  - これがメモリの二重解放エラーの例である

## エラーハンドリング

Rust では、デフォルトで `Option` 型と `Result` 型が存在する。

- Option

  - 値が存在しない(None)場合の例外処理を簡単に書くことができる。

- Result

  - 値がエラーが発生する可能性のある場合に簡単に書くことができる。

# 参考文献

- [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/index.html)
- [Rust の所有権システムが難しくて一度挫折したあの日の自分のためにわかりやすい読み物があるとよいと思ったのだ。](https://qiita.com/nirasan/items/9e169859c6807c2c175b)
