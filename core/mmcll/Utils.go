package mmcll

import (
	"crypto/sha1"
	"fmt"
	"io"
	"os"
	"path/filepath"
)

// GetFile 获取文件内容
func GetFile(path string) (string, error) {
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

// GetSha1 获取文件sha1
func GetSha1(path string) (string, error) {
	open, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer open.Close()
	hash := sha1.New()
	if _, err = io.Copy(hash, open); err != nil {
		return "", err
	}
	return fmt.Sprintf("%x", hash.Sum(nil)), nil
}

func SetFile(path, content string) error {
	// 确保目录存在
	dir := filepath.Dir(path)
	if dir != "" {
		if err := os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644)
	if err != nil {
		return err
	}
	defer file.Close()
	if _, err := file.WriteString(content); err != nil {
		return err
	}
	return nil
}
