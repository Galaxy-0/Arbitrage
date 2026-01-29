use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

// Dependencies needed in Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

const RATE_WORK: f64 = 1.0;
const RATE_RESEARCH: f64 = -4.0;

#[derive(Serialize, Deserialize, Debug)]
struct State {
    balance: f64,
    mode: String,
    start_time: f64,
}

impl Default for State {
    fn default() -> Self {
        State {
            balance: 0.0,
            mode: "IDLE".to_string(),
            start_time: 0.0,
        }
    }
}

fn get_data_file() -> PathBuf {
    let home = env::var("HOME").expect("Could not find HOME");
    PathBuf::from(home).join(".time_hedge_data.json")
}

fn load_state() -> State {
    let path = get_data_file();
    if !path.exists() {
        return State::default();
    }
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return State::default(),
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return State::default();
    }
    serde_json::from_str(&contents).unwrap_or_else(|_| State::default())
}

fn save_state(state: &State) {
    let path = get_data_file();
    let file = File::create(path).expect("Could not create file");
    serde_json::to_writer(file, state).expect("Could not write json");
}

fn get_current_time() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn get_current_balance(state: &State) -> f64 {
    if state.mode == "IDLE" {
        return state.balance;
    }
    let elapsed = (get_current_time() - state.start_time) / 3600.0;
    let rate = if state.mode == "RESEARCH" {
        RATE_RESEARCH
    } else {
        RATE_WORK
    };
    state.balance + (elapsed * rate)
}

fn format_duration(hours: f64) -> String {
    let sign = if hours < 0.0 {
        "-"
    } else if hours > 0.0 {
        "+"
    } else {
        " "
    };
    let abs_hours = hours.abs();
    let h = abs_hours as u64;
    let rem = abs_hours - (h as f64);
    let m = (rem * 60.0) as u64;
    let s = ((rem * 3600.0) % 60.0) as u64;
    format!("{}{:02}:{:02}:{:02}", sign, h, m, s)
}

fn handle_action(action: &str) {
    let mut state = load_state();
    state.balance = get_current_balance(&state);

    match action {
        "stop" => state.mode = "IDLE".to_string(),
        "research" => {
            state.mode = "RESEARCH".to_string();
            state.start_time = get_current_time();
        }
        "work" => {
            state.mode = "WORK".to_string();
            state.start_time = get_current_time();
        }
        "reset" => {
            state.balance = 0.0;
            state.mode = "IDLE".to_string();
        }
        _ => {}
    }
    save_state(&state);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        handle_action(&args[1]);
        return;
    }

    let state = load_state();
    let curr_bal = get_current_balance(&state);
    let formatted_time = format_duration(curr_bal);

    // 颜色逻辑：根据【状态】而非余额
    // Research = 花费 = 红
    // Work = 赚钱 = 绿
    // Idle = 暂停 = (如果有债则橙色警告，否则默认色)
    let color = if state.mode == "RESEARCH" {
        "#ff453a" // Red
    } else if state.mode == "WORK" {
        "#32d74b" // Green
    } else if curr_bal < 0.0 {
        "#ff9f0a" // Orange (Warning for debt)
    } else {
        "primary" // Default system color
    };

    // 覆盖逻辑：根据 State 决定最终图标
    let (final_sf_image, mode_text) = if state.mode == "IDLE" {
        ("pause.circle", "Idle")
    } else if state.mode == "RESEARCH" {
        ("atom", "Researching (4x Debt)")
    } else {
        ("keyboard", "Working (Payoff)")
    };

    // 顶栏输出
    if color == "primary" {
        println!("{} | sfimage={}", formatted_time, final_sf_image);
    } else {
        println!(
            "{} | color={} sfimage={}",
            formatted_time, color, final_sf_image
        );
    }

    println!("---");
    println!("current state: {}", mode_text);
    println!("current balance: {} | font=Menlo", formatted_time);
    println!("---");

    let exe = env::current_exe().unwrap();
    let exe_path = exe.to_string_lossy();

    // 菜单项也加上图标
    println!(
        "启动研究 (Research) | bash='{0}' param1='research' terminal=false refresh=true sfimage=atom",
        exe_path
    );
    println!(
        "启动搬砖 (Work) | bash='{0}' param1='work' terminal=false refresh=true sfimage=keyboard",
        exe_path
    );
    println!(
        "停止计时 (Stop) | bash='{0}' param1='stop' terminal=false refresh=true sfimage=pause.circle",
        exe_path
    );
    println!("---");
    println!(
        "债务清零 (Reset) | bash='{0}' param1='reset' terminal=false refresh=true sfimage=trash",
        exe_path
    );
}
