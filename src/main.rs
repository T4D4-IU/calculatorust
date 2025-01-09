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
        // トークン列に分割
        let tokens = Token::split(&line);
        // 式の評価
        match &tokens[0] {
            Token::MemoryPlus(memory_name) => {
                // メモリへの加算
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, prev_result);
                print_output(result);
            }
            Token::MemoryMinus(memory_name) => {
                // メモリへの減算
                let memory_name = memory_name.to_string();
                let result = memory.add(memory_name, -prev_result);
                print_output(result);
            }
            _ => {
                // 式の値の計算
                let left = eval_token(tokens[0], &memory);
                let right = eval_token(tokens[2], &memory);
                let result = eval_expression(left, tokens[1], right);
                // 結果の表示
                print_output(result);
                prev_result = result;
            }
        };
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

    fn add(&mut self, slot_name: String, prev_result: f64) -> f64 {
        // 全てのメモリを探索
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので、値を更新・表示して終了
                *entry.get_mut() += prev_result;
                *entry.get()
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので、新規作成・表示して終了
                entry.insert(prev_result);
                prev_result
            }
        }
    }
    fn get(&self, slot_name: &str) -> f64 {
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    MemoryRef(String),
    MemoryPlus(String),
    MemoryMinus(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl Token {
    fn parse(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,
            _ if value.starts_with("mem") => {
                let mut memory_name = value[3..].to_string();
                if value.ends_with('+') {
                    memory_name.pop(); // 末尾の１文字を削除
                    Self::MemoryPlus(memory_name)
                } else if value.ends_with('-') {
                    memory_name.pop(); // 末尾の１文字を削除
                    Self::MemoryMinus(memory_name)
                } else {
                    Self::MemoryRef(memory_name)
                }
            }
            _ => Self::Number(value.parse().unwrap()),
        }
    }
    fn split(line: &str) -> Vec<Self> {
        text.split(char::is_whitespace)
            .map(Self::parse)
            .collect()
    }
}

fn eval_token(token: &Token, memory: &Memory) -> f64 {
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
