package main

import (
	"io/fs"
	"os"
	"path/filepath"
)

func ensureConfigFile(path string) error {
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
	return nil
}
