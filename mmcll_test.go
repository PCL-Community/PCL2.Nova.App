package main

import (
	"NovaImitation/mmcll"
	"fmt"
	"strings"
	"testing"
)

func TestMMCLL(t *testing.T) {
	account := mmcll.NewLaunchAccountOffline("aooooo", "1234567980abcdef1234567890abcdef")
	options := *mmcll.NewLaunchOption(account, "D:/Languages/Java/jdk-21.0.7/bin/java.exe", "D:/Workspace/GoWork/", "", "")
	err := mmcll.LaunchGame(options, true, func(back []string) {
		fmt.Println(strings.Join(back, "\n"))
	})
	if err != nil {
		t.Error(err)
	}
}
