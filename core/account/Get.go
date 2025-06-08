package account

import (
	"NovaImitation/core/mmcll"
	"NovaImitation/core/utils"
	"encoding/json"
	"os"
	"path/filepath"
)

func (account *AccountBinding) GetAccountConfig() AccountList {
	home, err := utils.GetHomeDir()
	if err != nil {
		exePath, execErr := os.Executable()
		if execErr != nil {
			panic(execErr)
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
