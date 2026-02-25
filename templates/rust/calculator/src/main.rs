//! __SKILL_NAME__ — ZeroClaw Skill (Rust / WASI)
//!
//! Performs arithmetic: add, subtract, multiply, divide.
//! Protocol: read JSON from stdin, write JSON result to stdout.
//! Build:    cargo build --target wasm32-wasip1 --release
//!           cp target/wasm32-wasip1/release/__BIN_NAME__.wasm tool.wasm
//! Test:     zeroclaw skill test . --args '{"op":"add","a":3,"b":7}'

use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Args {
    op: String,
    a: f64,
    b: f64,
}

#[derive(Serialize)]
struct ToolResult {
    success: bool,
    output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<f64>,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let result = match serde_json::from_str::<Args>(&buf) {
        Ok(args) => calculate(args),
        Err(e) => ToolResult {
            success: false,
            output: String::new(),
            error: Some(format!(
                "invalid input: {e} — expected {{\"op\":\"add|sub|mul|div\",\"a\":1,\"b\":2}}"
            )),
            result: None,
        },
    };

    let out = serde_json::to_string(&result).unwrap();
    io::stdout().write_all(out.as_bytes()).unwrap();
}

fn calculate(args: Args) -> ToolResult {
    let (value, label) = match args.op.as_str() {
        "add" | "+" => (args.a + args.b, format!("{} + {}", args.a, args.b)),
        "sub" | "-" => (args.a - args.b, format!("{} - {}", args.a, args.b)),
        "mul" | "*" | "x" => (args.a * args.b, format!("{} × {}", args.a, args.b)),
        "div" | "/" => {
            if args.b == 0.0 {
                return ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some("division by zero".into()),
                    result: None,
                };
            }
            (args.a / args.b, format!("{} ÷ {}", args.a, args.b))
        }
        op => {
            return ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("unknown op '{op}' — use: add, sub, mul, div")),
                result: None,
            };
        }
    };

    ToolResult {
        success: true,
        output: format!("{label} = {value}"),
        error: None,
        result: Some(value),
    }
}
