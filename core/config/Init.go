package config

import (
	"io/fs"
	"os"
	"path/filepath"

	"github.com/pelletier/go-toml"
)

func EnsureConfigFile(path string) error {
	if _, err := os.Stat(path); err == nil {
		return nil // 文件已存在
	}
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, fs.ModePerm); err != nil {
		return err
	}
	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()
	
	defaultCfg := InitDefaultConfig()
	data, err := toml.Marshal(defaultCfg)
	if err != nil {
		return err
	}
	
	if _, err := file.Write(data); err != nil {
		return err
	}
	return nil
}

func InitDefaultConfig() *Config {
	cfg := &Config{}
	cfg.Customize.Theme.Name = "Nova"
	cfg.Customize.Theme.Mode = "Auto"
	return cfg
}

func (cb *ConfigBinding) LoadOrInitConfig() (*Config, error) {
	if err := EnsureConfigFile(cb.GetConfigTomlPath()); err != nil {
		return nil, err
	}
	return cb.ReadConfig()
}
