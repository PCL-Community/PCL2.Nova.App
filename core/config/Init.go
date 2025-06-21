package config

import (
	"io/fs"
	"log"
	"os"
	"path/filepath"

	"github.com/pelletier/go-toml"

	novafs "NovaImitation/core/fs"
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
	exePath, err := novafs.GetExecutableDir()
	if err != nil {
		log.Fatalf("无法获取可执行文件路径: %v", err)
	}
	if err := novafs.EnsureDir(filepath.Join(exePath, ".minecraft", "")); err != nil {
		log.Fatalf("无法确保目录存在: %v", err)
	}
	cfg.ProfileFolder = append(cfg.ProfileFolder, ProfileFolder{
		Name:    "当前文件夹",
		AbsPath: filepath.Join(exePath, ".minecraft"),
	})
	return cfg
}

func (cb *ConfigBinding) LoadOrInitConfig() (*Config, error) {
	if err := EnsureConfigFile(cb.GetConfigTomlPath()); err != nil {
		return nil, err
	}
	return cb.ReadConfig()
}
