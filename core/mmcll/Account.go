package mmcll

import (
	"crypto/md5"
	"fmt"
)

// GenerateBukkitUUID 生成一个 Username 通过 Bukkit 生成的标准 UUID
func GenerateBukkitUUID(username string) string {
	username = "OfflinePlayer:" + username
	data := []byte(username)
	hash := md5.Sum(data)
	hash[6] = (hash[6] & 0x0f) | 0x30
	hash[8] = (hash[8] & 0x3f) | 0x80
	return fmt.Sprintf("%x", hash)
}

// NewLaunchAccountOffline 新建一个离线登录模块
func NewLaunchAccountOffline(name, uuid string) launchAccount {
	return launchAccount{
		name:        name,
		uuid:        uuid,
		accessToken: uuid,
		atype:       "Legacy",
		base:        "",
		url:         "",
		online:      1,
	}
}

// NewLaunchAccountMicrosoft 新建一个微软登录模块
func NewLaunchAccountMicrosoft(name, uuid, accessToken string) launchAccount {
	return launchAccount{
		name:        name,
		uuid:        uuid,
		accessToken: accessToken,
		atype:       "msa",
		base:        "",
		url:         "",
		online:      2,
	}
}

// NewLaunchAccountThirdParty 新建一个外置登录模块
func NewLaunchAccountThirdParty(name, uuid, accessToken, base, url string) launchAccount {
	return launchAccount{
		name:        name,
		uuid:        uuid,
		accessToken: accessToken,
		atype:       "msa",
		base:        base,
		url:         url,
		online:      3,
	}
}

func (a launchAccount) GetName() string {
	return a.name
}
func (a launchAccount) GetUUID() string {
	return a.uuid
}
func (a launchAccount) GetAccessToken() string {
	return a.accessToken
}
func (a launchAccount) GetAtype() string {
	return a.atype
}
func (a launchAccount) GetBase() string {
	return a.base
}
func (a launchAccount) GetUrl() string {
	return a.url
}
func (a launchAccount) GetOnline() int8 {
	return a.online
}
