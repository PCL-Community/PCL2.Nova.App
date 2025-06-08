package config

import (
	"os"
	"path/filepath"
)

func GetCurrentExeDir() string {
	exePath, err := os.Executable()
	if err != nil {
		panic(err)
	}
	return filepath.Dir(exePath)
}

func (cb *ConfigBinding) GetConfigTomlPath() string {
	res := filepath.Join(GetCurrentExeDir(), "PCL.Nova", "Config.toml")
	err := EnsureConfigFile(res)
	if err != nil {
		panic(err)
	}
	return res
}
