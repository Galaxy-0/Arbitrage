package main

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"time"
)

// === 配置区 ===
const (
	rateWork     = 1.0
	rateResearch = -4.0
)

var dataFile = filepath.Join(os.Getenv("HOME"), ".time_hedge_data.json")

// State 定义状态结构
type State struct {
	Balance   float64 `json:"balance"`
	Mode      string  `json:"mode"`
	StartTime float64 `json:"start_time"`
}

func loadState() State {
	f, err := os.Open(dataFile)
	if err != nil {
		return State{Balance: 0.0, Mode: "IDLE", StartTime: 0}
	}
	defer f.Close()

	var state State
	if err := json.NewDecoder(f).Decode(&state); err != nil {
		return State{Balance: 0.0, Mode: "IDLE", StartTime: 0}
	}
	return state
}

func saveState(state State) {
	f, err := os.Create(dataFile)
	if err != nil {
		return
	}
	defer f.Close()
	json.NewEncoder(f).Encode(state)
}

func getCurrentBalance(state State) float64 {
	if state.Mode == "IDLE" {
		return state.Balance
	}

	elapsedHours := (float64(time.Now().UnixNano())/1e9 - state.StartTime) / 3600.0
	rate := rateWork
	if state.Mode == "RESEARCH" {
		rate = rateResearch
	}
	return state.Balance + (elapsedHours * rate)
}

func formatDuration(hours float64) string {
	sign := " "
	if hours < 0 {
		sign = "-"
	} else if hours > 0 {
		sign = "+"
	}
	
	absHours := math.Abs(hours)
	h := int(absHours)
	m := int((absHours * 60) - float64(h*60))
	s := int((absHours * 3600) - float64(h*3600) - float64(m*60))
	
	return fmt.Sprintf("%s%02d:%02d:%02d", sign, h, m, s)
}

func handleAction(action string) {
	state := loadState()

	// 结算当前
	state.Balance = getCurrentBalance(state)

	if action == "stop" {
		state.Mode = "IDLE"
	} else if action == "research" {
		state.Mode = "RESEARCH"
		state.StartTime = float64(time.Now().UnixNano()) / 1e9
	} else if action == "work" {
		state.Mode = "WORK"
		state.StartTime = float64(time.Now().UnixNano()) / 1e9
	} else if action == "reset" {
		state.Balance = 0.0
		state.Mode = "IDLE"
	}

	saveState(state)
}

func main() {
	if len(os.Args) > 1 {
		handleAction(os.Args[1])
		return
	}

	state := loadState()
	currBal := getCurrentBalance(state)
	formattedTime := formatDuration(currBal)

	color := "green"
	icon := "🟢"
	if currBal < 0 {
		color = "red"
		icon = "🔴"
	}

	modeText := ""
	if state.Mode == "IDLE" {
		icon = "⚪️"
		modeText = "Idle"
	} else if state.Mode == "RESEARCH" {
		icon = "🧪"
		modeText = "Researching (4x Debt)"
	} else {
		icon = "🔨"
		modeText = "Working (Payoff)"
	}

	fmt.Printf("%s %s | color=%s\n", icon, formattedTime, color)
	fmt.Println("---")
	fmt.Printf("当前状态: %s\n", modeText)
	fmt.Printf("当前余额: %s (%.4fh)\n", formattedTime, currBal)
	fmt.Println("---")

	exe, _ := os.Executable()
	fmt.Printf("🧪 启动研究 (Research) | bash='%s' param1='research' terminal=false refresh=true\n", exe)
	fmt.Printf("🔨 启动搬砖 (Work) | bash='%s' param1='work' terminal=false refresh=true\n", exe)
	fmt.Printf("⏸ 停止计时 (Stop) | bash='%s' param1='stop' terminal=false refresh=true\n", exe)
	fmt.Println("---")
	fmt.Printf("♻️ 债务清零 (Reset) | bash='%s' param1='reset' terminal=false refresh=true\n", exe)
}
