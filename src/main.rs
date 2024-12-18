use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();
        // メモリへの書き込み
        if tokens[0] == "mem+" {
            memory += prev_result;
            print_output(memory);
            continue;
        } else if tokens[0] == "mem-" {
            memory -= prev_result;
            print_output(memory);
            continue;
        }
        // 式の計算
        let left: f64 = eval_token(tokens[0], memory);
        let right: f64 = eval_token(tokens[2], memory);
        let result = eval_expression(left, tokens[1], right);
        // 結果を表示
        print_output(result);

        prev_result = result;
    }
}

fn print_output(value: f64) {
    println!(" => {}", value);
}

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, token: &str, right: f64) -> f64 {
    match token {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            // 入力が不正な場合
            unreachable!();
        }
    }
}