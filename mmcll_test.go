package main

import (
	"NovaImitation/launcher"
	"NovaImitation/mmcll"
	"fmt"
	"strings"
	"testing"
)

func TestMMCLL(t *testing.T) {
	account := mmcll.NewLaunchAccountOffline("aooooo", "1234567980abcdef1234567890abcdef")
	options := *mmcll.NewLaunchOption(account, "D:/Languages/Java/jdk-21.0.7/bin/java.exe", "D:/Workspace/GoWork/", "D:/Workspace/GoWork/", "D:/Workspace/GoWork/")
	err := mmcll.LaunchGame(options, true, func(back []string) {
		fmt.Println(strings.Join(back, "\n"))
	})
	if err != nil {
		t.Error(err)
	}
}
func stringPtr(s string) *string {
	return &s
}
func getAccessToken(a launcher.AccountType) string {
	if a.AccessToken != nil {
		return *a.AccessToken
	}
	return ""
}
func TestOther(t *testing.T) {
	al := &launcher.AccountList{
		Accounts: []launcher.AccountType{
			{
				Name:     "Steve",
				UUID:     "1234567890",
				AType:    "Offline",
				HeadSkin: "123456",
			},
			{
				Name:         "Steve",
				UUID:         "1234567890",
				AType:        "Microsoft",
				HeadSkin:     "123456",
				AccessToken:  stringPtr("AT"),
				RefreshToken: stringPtr("RT"),
			},
			{
				Name:        "Steve",
				UUID:        "1234567890",
				AType:       "Thirdparty",
				HeadSkin:    "123456",
				AccessToken: stringPtr("AT"),
				ClientToken: stringPtr("CT"),
				Server:      stringPtr("SV"),
				BaseCode:    stringPtr("123456"),
			},
		},
	}
	at := launcher.Account{}
	at.SetAccountConfig(*al)
	for _, r := range at.GetAccountConfig().Accounts {
		fmt.Println(r)
	}
}
