# 项目说明
## 说明
创建一个Rust工程，

（1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
（2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
（3）使用Cargo编译运行此工程

（需要自行发现其中的细节，一个考点是：ascii码字符的顺序）

## 输出结果
`
a-Z
'a' '`' '_' '^' ']' '\\' '[' 'Z'
A-z
'A' 'B' 'C' 'D' 'E' 'F' 'G' 'H' 'I' 'J' 'K' 'L' 'M' 'N' 'O' 'P' 'Q' 'R' 'S' 'T' 'U' 'V' 'W' 'X' 'Y' 'Z' '[' '\\' ']' '^' '_' '`' 'a' 'b' 'c' 'd' 'e' 'f' 'g' 'h' 'i' 'j' 'k' 'l' 'm' 'n' 'o' 'p' 'q' 'r' 's' 't' 'u' 'v' 'w' 'x' 'y' 'z'

`
## 运行方式
输入以下命令
`
cargo run --color=always --package rust-lession --bin rust-lession
`