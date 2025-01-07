use std::collections::{hash_map::Entry, HashMap};
use std::io::stdin;

fn main() {
    let mut memory = Memory::new();
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
    slots: HashMap<String, f64>,
}

impl Memory {
    fn new()-> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    fn add_and_print(&mut self, token: &str, prev_result: f64) {
        let slot_name = token[3..token.len() -1].to_string();
        // 全てのメモリを探索
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので、値を更新・表示して終了
                *entry.get_mut() += prev_result;
                print_output(*entry.get());
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので、新規作成・表示して終了
                entry.insert(prev_result);
                print_output(prev_result);
            }
        }
    }

    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            // self.slots.get(slot_name)の戻り値はOption<&f64>
            // Optionの中身が参照のままでは値を返すことができない
            // copied()で値をコピーしてOption<f64>に変換する
            // unwrap_or()でOptionがNoneの場合に0.0を返す
            self.slots.get(slot_name).copied().unwrap_or(0.0)
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
