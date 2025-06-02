package launcher

import (
	"NovaImitation/info"
	"NovaImitation/mmcll"
	"encoding/json"
	"os"
	"path/filepath"
)

type AccountType struct {
	AType        string  `json:"type"`
	Name         string  `json:"name"`
	UUID         string  `json:"uuid"`
	AccessToken  *string `json:"access_token,omitempty"`
	RefreshToken *string `json:"refresh_token,omitempty"`
	ClientToken  *string `json:"client_token,omitempty"`
	Server       *string `json:"server,omitempty"`
	BaseCode     *string `json:"base_code,omitempty"`
	HeadSkin     string  `json:"head_skin"`
}
type AccountList struct {
	Accounts []AccountType `json:"accounts"`
}
type Account struct{}

func (account *Account) SetAccountConfig(accounts AccountList) {
	realJson, err := json.MarshalIndent(accounts, "", "\t")
	if err != nil {
		panic(err)
	}
	str := string(realJson)
	home, err := info.GetHomeDir()
	if err != nil {
		exePath, err := os.Executable()
		if err != nil {
			panic(err)
		}
		home = filepath.Join(filepath.Dir(exePath), "PCL.Nova", "config")
	}
	err = mmcll.SetFile(filepath.Join(home, "AccountJSON.json"), str)
	if err != nil {
		panic(err)
	}
}

func (account *Account) GetAccountConfig() AccountList {
	home, err := info.GetHomeDir()
	if err != nil {
		exePath, err := os.Executable()
		if err != nil {
			panic(err)
		}
		home = filepath.Join(filepath.Dir(exePath), "PCL.Nova", "config")
	}
	jsonContent, err := mmcll.GetFile(filepath.Join(home, "AccountJSON.json"))
	if err != nil {
		return AccountList{}
	}
	var at *AccountList
	err = json.Unmarshal([]byte(jsonContent), &at)
	if err != nil {
		return AccountList{}
	}
	return *at
}
