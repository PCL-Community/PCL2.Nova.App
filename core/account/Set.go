package account

import (
	"NovaImitation/core/mmcll"
	"NovaImitation/core/utils"
	"encoding/json"
	"os"
	"path/filepath"
)

func (account *AccountBinding) SetAccountConfig(accounts AccountList) {
	realJson, err := json.MarshalIndent(accounts, "", "\t")
	if err != nil {
		panic(err)
	}
	str := string(realJson)
	home, err := utils.GetHomeDir()
	if err != nil {
		exePath, execErr := os.Executable()
		if execErr != nil {
			panic(execErr)
		}
		home = filepath.Join(filepath.Dir(exePath), "PCL.Nova", "config")
	}
	err = mmcll.SetFile(filepath.Join(home, "AccountJSON.json"), str)
	if err != nil {
		panic(err)
	}
}
