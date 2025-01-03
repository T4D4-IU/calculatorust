use std::io::stdin;

fn main() {
    let mut memory = Memory {
        slots: vec![0.0; 10],
    };
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
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            add_and_print_memory(&mut memory, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(&mut memory, tokens[0], -prev_result);
            continue;
        }
        // 式の計算
        let left = eval_token(tokens[0], &memory);
        let right = eval_token(tokens[2], &memory);
        let result = eval_expression(left, tokens[1], right);
        // 結果を表示
        print_output(result);

        prev_result = result;
    }
}

struct Memory {
    slots: Vec<f64>,
}

fn add_and_print_memory(memory: &mut Memory, token: &str, prev_result: f64) {
    let slot_index: usize = token[3..token.len() -1].parse().unwrap();
    memory.slots[slot_index] += prev_result;
    print_output(memory.slots[slot_index]);
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..].parse().unwrap();
        memory.slots[slot_index]
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

fn print_output(value: f64) {
    println!(" => {}", value);
}
