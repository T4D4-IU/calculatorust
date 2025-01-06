use std::io::stdin;

fn main() {
    let mut memory = Memory {
        slots: vec![],
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
            memory.add_and_print(tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            memory.add_and_print( tokens[0], -prev_result);
            continue;
        }
        // 式の計算
        let left = memory.eval_token(tokens[0]);
        let right = memory.eval_token(tokens[2]);
        let result = eval_expression(left, tokens[1], right);
        // 結果を表示
        print_output(result);

        prev_result = result;
    }
}

struct Memory {
    slots: Vec<(String, f64)>,
}

impl Memory {
    fn add_and_print(&mut self, token: &str, prev_result: f64) {
        let slot_name = &token[3..token.len() -1];
        // 全てのメモリを探索
        for slot in self.slots.iter_mut() {
            if slot.0 == slot_name {
                // メモリが見つかったので値を更新・表示して終了
                slot.1 += prev_result;
                print_output(slot.1);
                return;
            }
        }
        // メモリが見つからなかったので最後に追加
        self.slots.push((slot_name.to_string(), prev_result));
        print_output(prev_result);
    }

    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            for slot in &self.slots {
                if slot.0 == slot_name {
                    return slot.1;
                }
            }
            // メモリが見つからなかった場合
            0.0
        } else {
            token.parse().unwrap()
        }
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
