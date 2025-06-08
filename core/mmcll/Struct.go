package mmcll

type errorMMCLL struct {
	code int32  // code 报错代码
	msg  string // msg 报错信息
}

// launchAccount 用来登录账号所需要的所有键，仅能 new 一次，后续无法修改。
type launchAccount struct {
	name        string // name 账户名字
	uuid        string // uuid 账号UUID
	accessToken string // accessToken 正版账号登录密钥
	atype       string // atype 账号登录类型
	base        string // base 账号登录第三方数据（仅限外置登录）
	url         string // url 账号登录第三方网址（仅限外置登录）
	online      int8   // online 账号类型（1：离线、2：正版、3：外置）
}

// launchOption 启动设置类（新建后无法修改 account、javaPath、rootPath、versionPath 几个必填项。）
type launchOption struct {
	Account        launchAccount
	javaPath       string
	rootPath       string
	versionPath    string
	gamePath       string
	windowHeight   uint32
	windowWidth    uint32
	minMemory      uint32
	maxMemory      uint32
	customInfo     string
	additionalJvm  string
	additionalGame string
}

// launchGame 正式启动游戏的类
type launchGame struct {
	account        launchAccount
	javaPath       string
	rootPath       string
	versionPath    string
	gamePath       string
	windowHeight   uint32
	windowWidth    uint32
	minMemory      uint32
	maxMemory      uint32
	customInfo     string
	additionalJvm  string
	additionalGame string
	callback       func([]string)
}
