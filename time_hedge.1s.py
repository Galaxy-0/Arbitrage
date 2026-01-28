#!/usr/bin/env python3

# <bitbar.title>Time Hedge</bitbar.title>
# <bitbar.version>v1.0</bitbar.version>
# <bitbar.author>YourName</bitbar.author>
# <bitbar.desc>Research vs Work Debt Manager</bitbar.desc>

import sys
import os
import json
import time

# === 配置区 ===
DATA_FILE = os.path.expanduser("~/.time_hedge_data.json")
RATE_WORK = 1.0       # 搬砖 1 小时 = +1 余额
RATE_RESEARCH = -4.0  # 研究 1 小时 = -4 余额 (债务)

# === 核心逻辑 ===

def load_state():
    if not os.path.exists(DATA_FILE):
        return {"balance": 0.0, "mode": "IDLE", "start_time": 0}
    with open(DATA_FILE, "r") as f:
        return json.load(f)

def save_state(state):
    with open(DATA_FILE, "w") as f:
        json.dump(state, f)

def get_current_balance(state):
    if state["mode"] == "IDLE":
        return state["balance"]
    
    elapsed_hours = (time.time() - state["start_time"]) / 3600.0
    rate = RATE_RESEARCH if state["mode"] == "RESEARCH" else RATE_WORK
    return state["balance"] + (elapsed_hours * rate)

def handle_action(action):
    state = load_state()
    
    # 先结算当前段落
    state["balance"] = get_current_balance(state)
    
    if action == "stop":
        state["mode"] = "IDLE"
    elif action == "research":
        state["mode"] = "RESEARCH"
        state["start_time"] = time.time()
    elif action == "work":
        state["mode"] = "WORK"
        state["start_time"] = time.time()
    elif action == "reset":
        state["balance"] = 0.0
        state["mode"] = "IDLE"
        
    save_state(state)

# === 如果是点击事件触发，处理完直接退出 ===
if len(sys.argv) > 1:
    handle_action(sys.argv[1])
    sys.exit()


def format_duration(hours):
    """将小时数转换为 [+/-]HH:MM:SS 格式"""
    sign = "-" if hours < 0 else "+" if hours > 0 else " "
    abs_hours = abs(hours)
    
    h = int(abs_hours)
    m = int((abs_hours * 60) % 60)
    s = int((abs_hours * 3600) % 60)
    
    return f"{sign}{h:02d}:{m:02d}:{s:02d}"

# === 渲染 UI (每秒刷新) ===
state = load_state()
curr_bal = get_current_balance(state)
formatted_time = format_duration(curr_bal)

# 颜色与图标逻辑
color = "red" if curr_bal < 0 else "green"
icon = "🔴" if curr_bal < 0 else "🟢"
if state["mode"] == "IDLE":
    icon = "⚪️"
    mode_text = "Idle"
elif state["mode"] == "RESEARCH":
    icon = "🧪" # 研究中
    mode_text = "Researching (4x Debt)"
else:
    icon = "🔨" # 搬砖中
    mode_text = "Working (Payoff)"

# 顶栏显示内容
print(f"{icon} {formatted_time} | color={color}")

# 下拉菜单内容
print("---")
print(f"当前状态: {mode_text}")
print(f"当前余额: {formatted_time} ({curr_bal:.4f}h)")
print("---")
# bash参数指定脚本路径，param1传参给脚本
script_path = os.path.abspath(__file__)
print(f"🧪 启动研究 (Research) | bash='{sys.executable}' param1='{script_path}' param2='research' terminal=false refresh=true")
print(f"🔨 启动搬砖 (Work) | bash='{sys.executable}' param1='{script_path}' param2='work' terminal=false refresh=true")
print(f"⏸ 停止计时 (Stop) | bash='{sys.executable}' param1='{script_path}' param2='stop' terminal=false refresh=true")
print("---")
print(f"♻️ 债务清零 (Reset) | bash='{sys.executable}' param1='{script_path}' param2='reset' terminal=false refresh=true")
