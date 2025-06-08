package config

import (
	"github.com/pelletier/go-toml"
	"os"
)

func (cb *ConfigBinding) ReadConfig() (*Config, error) {
	data, err := os.ReadFile(cb.GetConfigTomlPath())
	if err != nil {
		return nil, err
	}

	var cfg Config
	if err := toml.Unmarshal(data, &cfg); err != nil {
		return nil, err
	}

	return &cfg, nil
}
