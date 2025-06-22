package main

import (
	"NovaImitation/core/account"
	"NovaImitation/core/config"
	"NovaImitation/core/network"

	"context"
	"embed"
	"time"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
	"github.com/wailsapp/wails/v2/pkg/runtime"
)

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

func (a *App) StartDownload() {
	for progress := 10; progress <= 100; progress += 10 {
		time.Sleep(time.Second)
		runtime.EventsEmit(a.ctx, "download_progress", progress)
	}
	runtime.EventsEmit(a.ctx, "download_success")
}

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	// Create an instance of the app structure
	app := NewApp()

	// Create application with options
	err := wails.Run(&options.App{
		Title:     "PCL2.Nova.App",
		Width:     1024,
		Height:    614,
		MinWidth:  1024,
		MinHeight: 614,
		Frameless: true,
		AssetServer: &assetserver.Options{
			Assets: assets,
		},
		OnStartup: app.startup,
		Bind: []any{
			app,
			&network.Network{},
			&account.AccountBinding{},
			&config.ConfigBinding{},
		},
	})

	if err != nil {
		println("Error:", err.Error())
	}
}
