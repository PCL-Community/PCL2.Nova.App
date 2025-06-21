package fs

import (
	"fmt"
	"os"
	"path/filepath"
)

func GetExecutableDir() (string, error) {
	exePath, err := os.Executable()
	if err != nil {
		return "", fmt.Errorf("无法获取可执行文件路径: %w", err)
	}
	exePath, err = filepath.EvalSymlinks(exePath) // 解析符号链接（如果有）
	if err != nil {
		return "", fmt.Errorf("无法解析符号链接: %w", err)
	}
	return filepath.Dir(exePath), nil
}

func EnsureDir(path string) error {
	info, err := os.Stat(path)
	if os.IsNotExist(err) {
		return os.MkdirAll(path, os.ModePerm)
	}
	if err != nil {
		return err // 其他错误（如权限问题）应当上报
	}
	if !info.IsDir() {
		return fmt.Errorf("%s exists but is not a directory", path)
	}
	return nil
}
