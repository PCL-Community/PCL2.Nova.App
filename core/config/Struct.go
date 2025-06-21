package config

type Config struct {
	Customize     CustomizeConfig `toml:"Customize"`
	ProfileFolder []ProfileFolder `toml:"ProfileFolder"`
}

type ThemeConfig struct {
	Name string `toml:"Name"`
	Mode string `toml:"Mode"`
}

type CustomizeConfig struct {
	Theme ThemeConfig `toml:"Theme"`
}

type ProfileFolder struct {
	Name    string `toml:"Name"`
	AbsPath string `toml:"AbsPath"`
}

type ConfigBinding struct{}
