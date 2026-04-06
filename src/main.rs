mod calc;

fn main() {
    println!("欢迎使用 rust 计算器");
    println!("输入算数表达式，按回车计算。 支持的运算符有 + - * / ^ () 符号。");
    println!("输入 q 退出");
    
    loop {
        let mut input = String::new();
        println!("请输入表达式：");
        std::io::stdin().read_line(&mut input).expect("无法读取输入");
        let input = input.trim();
        if input == "q" {
            println!("程序已退出，欢迎下次使用，再见");
            break;
        }else if input.is_empty() {
            continue;
        }
        match calc::calculate(&input) {
            Ok(res) => println!("结果为：{}", res),
            Err(err) => println!("计算错误：{}", err),
        }

    }
}
