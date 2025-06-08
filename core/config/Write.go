package config

import (
	"github.com/pelletier/go-toml"
	"os"
)

func (cb *ConfigBinding) WriteConfig(cfg *Config) error {
	data, err := toml.Marshal(cfg)
	if err != nil {
		return err
	}

	return os.WriteFile(cb.GetConfigTomlPath(), data, 0644)
}
