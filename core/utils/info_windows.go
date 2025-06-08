package utils

import (
	"os/user"
	"path/filepath"

	"golang.org/x/sys/windows"
)

func GetWindowsVersion() bool {
	info := windows.RtlGetVersion()
	if info == nil {
		return false
	}
	if info.MajorVersion >= 10 {
		return true
	}
	return false
}

func GetHomeDir() (string, error) {
	// 获取当前用户
	currentUser, err := user.Current()
	if err != nil {
		return "", err
	}
	return filepath.Join(currentUser.HomeDir, "AppData", "Roaming", "PCL.Nova"), nil
}
