package config

type ThemeConfig struct {
	Name string `toml:"Name"`
	Mode string `toml:"Mode"`
}

type CustomizeConfig struct {
	Theme ThemeConfig `toml:"Theme"`
}

type Config struct {
	Customize CustomizeConfig `toml:"Customize"`
}

type ConfigBinding struct{}
