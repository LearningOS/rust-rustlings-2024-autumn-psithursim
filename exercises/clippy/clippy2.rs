// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res: i32 = 42;
    let mut option = Some(12); // 使 option 可变

    while let Some(x) = option {
        if let Some(new_res) = res.checked_add(x) {
            res = new_res;
        } else {
            eprintln!("Error: integer overflow");
            break; // 或者你可以选择其他处理方式
        }
        option = Some(option.unwrap() - 1); // 每次迭代减少 1
    }
    println!("{}", res);
}