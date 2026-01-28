最简单的原型就是利用 macOS 生态中最著名的神器 **SwiftBar**（开源免费），配合一段**不到 50 行的 Python 脚本**。

不需要编译，不需要复杂的环境配置，**复制 -> 粘贴 -> 运行** 即可。

### 核心原理

1. **SwiftBar** 负责在 Mac 顶栏渲染文字和处理点击事件。
2. **脚本** 负责计算“债务”逻辑：
* **当前余额** = (上次结束时的余额) + (本次时长 × 杠杆率)
* **杠杆率**：搬砖 = +1.0 / 研究 = -4.0


ß
---

### 第一步：安装 SwiftBar

如果你还没装，去 GitHub Release 页下载或者直接用 brew：

```bash
brew install --cask swiftbar

```

安装后打开，它会让你选一个文件夹作为“插件目录”。随便建一个文件夹，比如 `~/Documents/SwiftBarPlugins`。

---

### 第二步：创建脚本 (最简原型)

在刚才那个插件目录下，新建一个文件，命名为 **`time_hedge.1s.py`**。
*(注意：文件名里的 `.1s.` 表示每秒刷新一次，保证倒计时看着爽。)*

把下面的代码完整复制进去：

```python
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

# === 渲染 UI (每秒刷新) ===
state = load_state()
curr_bal = get_current_balance(state)

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
print(f"{icon} {curr_bal:.4f}h | color={color}")

# 下拉菜单内容
print("---")
print(f"当前状态: {mode_text}")
print(f"当前余额: {curr_bal:.4f} 小时")
print("---")
# bash参数指定脚本路径，param1传参给脚本
script_path = os.path.abspath(__file__)
print(f"🧪 启动研究 (Research) | bash='{sys.executable}' param1='{script_path}' param2='research' terminal=false refresh=true")
print(f"🔨 启动搬砖 (Work) | bash='{sys.executable}' param1='{script_path}' param2='work' terminal=false refresh=true")
print(f"⏸ 停止计时 (Stop) | bash='{sys.executable}' param1='{script_path}' param2='stop' terminal=false refresh=true")
print("---")
print(f"♻️ 债务清零 (Reset) | bash='{sys.executable}' param1='{script_path}' param2='reset' terminal=false refresh=true")

```

---

### 第三步：赋予权限

打开终端，运行这一行命令给它执行权限：

```bash
chmod +x ~/Documents/SwiftBarPlugins/time_hedge.1s.py

```

*(路径记得换成你自己的)*

### 效果展示

现在你的 Mac 顶栏右上角应该会出现一个数字。

1. **初始状态：** `⚪️ 0.0000h`
2. **点击下拉菜单，选 "🧪 启动研究"：**
* 数字开始狂跳，每秒钟都在**减少**（因为是负债）。
* 你会眼睁睁看着 `-0.0011h` 变成 `-0.0044h`... 这种看着数字流逝的压迫感非常适合控制好奇心。


3. **选 "🔨 启动搬砖"：**
* 数字开始缓慢回升（正向积累）。


4. **选 "⏸ 停止"：**
* 数字定格，等待你下次操作。



这就是最简单的 **MVP (Minimum Viable Product)**。只要你会 Python，你甚至可以在里面加个逻辑：*如果欠债超过 10 小时，顶栏字体变成红色加粗并弹窗报警。*

