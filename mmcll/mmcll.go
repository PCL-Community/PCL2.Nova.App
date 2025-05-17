package mmcll

import (
	"NovaImitation/info"
	"errors"
	"fmt"
	"github.com/go-vgo/robotgo"
	"github.com/shirou/gopsutil/mem"
	"os"
	"regexp"
	"runtime"
)

// SomeConst.go

const (
	// LauncherName 启动器名称（请自行修改）
	LauncherName = "MMCLL"
	// LauncherVersion 启动器版本
	LauncherVersion = "0.0.1-Alpha-12"
)

// MMCLLError 定义报错类型
type MMCLLError struct {
	// code 报错代码
	code int32
	// msg 报错信息
	msg string
}

func (e *MMCLLError) Error() string {
	return fmt.Sprintf("Err Code: %d, Err Message: %s", e.code, e.msg)
}
func NewMMCLLError(code int32, msg string) *MMCLLError {
	return &MMCLLError{code, msg}
}

// MainMethod.go

// LaunchMethod.go

// LaunchAccount 用来登录账号所需要的所有键，仅能new一次，后续无法修改。
type LaunchAccount struct {
	// name 账户名字
	name string
	// uuid 账号UUID
	uuid string
	// accessToken 正版账号登录密钥
	accessToken string
	// atype 账号登录类型
	atype string
	// base 账号登录第三方数据（仅限外置登录）
	base string
	// url 账号登录第三方网址（仅限外置登录）
	url string
	// online 账号类型（1：离线、2：正版、3：外置）
	online int8
}

// NewLaunchAccountOffline 新建一个离线登录模块
func NewLaunchAccountOffline(name string, uuid string) LaunchAccount {
	return LaunchAccount{
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
func NewLaunchAccountMicrosoft(name string, uuid string, accessToken string) LaunchAccount {
	return LaunchAccount{
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
func NewLaunchAccountThirdParty(name string, uuid string, accessToken string, base string, url string) LaunchAccount {
	return LaunchAccount{
		name:        name,
		uuid:        uuid,
		accessToken: accessToken,
		atype:       "msa",
		base:        base,
		url:         url,
		online:      3,
	}
}
func (a LaunchAccount) GetName() string {
	return a.name
}
func (a LaunchAccount) GetUUID() string {
	return a.uuid
}
func (a LaunchAccount) GetAccessToken() string {
	return a.accessToken
}
func (a LaunchAccount) GetAtype() string {
	return a.atype
}
func (a LaunchAccount) GetBase() string {
	return a.base
}
func (a LaunchAccount) GetUrl() string {
	return a.url
}
func (a LaunchAccount) GetOnline() int8 {
	return a.online
}

// LaunchOption 启动设置类（新建后无法修改account、javaPath、rootPath、versionPath几个必填项。）
type LaunchOption struct {
	account        LaunchAccount
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

// NewLaunchOption 新建一个启动设置类。（以下非必填的可以直接链式调用设置初始值）
func NewLaunchOption(account LaunchAccount, javaPath string, rootPath string, versionPath string, gamePath string) *LaunchOption {
	return &LaunchOption{
		account:        account,
		javaPath:       javaPath,
		rootPath:       rootPath,
		versionPath:    versionPath,
		gamePath:       gamePath,
		windowHeight:   480,
		windowWidth:    854,
		minMemory:      256,
		maxMemory:      1024,
		customInfo:     LauncherName + "-" + LauncherVersion,
		additionalJvm:  "",
		additionalGame: "",
	}
}
func (opt *LaunchOption) SetWindowWidth(windowWidth uint32) *LaunchOption {
	opt.windowWidth = windowWidth
	return opt
}
func (opt *LaunchOption) SetWindowHeight(windowHeight uint32) *LaunchOption {
	opt.windowHeight = windowHeight
	return opt
}
func (opt *LaunchOption) SetMinMemory(minMemory uint32) *LaunchOption {
	opt.minMemory = minMemory
	return opt
}
func (opt *LaunchOption) SetMaxMemory(maxMemory uint32) *LaunchOption {
	opt.maxMemory = maxMemory
	return opt
}
func (opt *LaunchOption) SetCustomInfo(customInfo string) *LaunchOption {
	opt.customInfo = customInfo
	return opt
}
func (opt *LaunchOption) SetAdditionalJvm(additionalJvm string) *LaunchOption {
	opt.additionalJvm = additionalJvm
	return opt
}
func (opt *LaunchOption) SetAdditionalGame(additionalGame string) *LaunchOption {
	opt.additionalGame = additionalGame
	return opt
}
func (opt *LaunchOption) GetAccount() LaunchAccount {
	return opt.account
}
func (opt *LaunchOption) GetJavaPath() string {
	return opt.javaPath
}
func (opt *LaunchOption) GetRootPath() string {
	return opt.rootPath
}
func (opt *LaunchOption) GetVersionPath() string {
	return opt.versionPath
}
func (opt *LaunchOption) GetGamePath() string {
	return opt.gamePath
}
func (opt *LaunchOption) GetWindowHeight() uint32 {
	return opt.windowHeight
}
func (opt *LaunchOption) GetWindowWidth() uint32 {
	return opt.windowWidth
}
func (opt *LaunchOption) GetMinMemory() uint32 {
	return opt.minMemory
}
func (opt *LaunchOption) GetMaxMemory() uint32 {
	return opt.maxMemory
}
func (opt *LaunchOption) GetCustomInfo() string {
	return opt.customInfo
}
func (opt *LaunchOption) GetAdditionalJvm() string {
	return opt.additionalJvm
}
func (opt *LaunchOption) GetAdditionalGame() string {
	return opt.additionalGame
}

// launchGame 正式启动游戏的类
type launchGame struct {
	account        LaunchAccount
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

// newLaunchStart 初始化启动类
func newLaunchStart(option LaunchOption, callback func([]string)) launchGame {
	ls := launchGame{
		account:        option.account,
		javaPath:       option.javaPath,
		rootPath:       option.rootPath,
		versionPath:    option.versionPath,
		gamePath:       option.gamePath,
		windowHeight:   option.windowHeight,
		windowWidth:    option.windowWidth,
		minMemory:      option.minMemory,
		maxMemory:      option.maxMemory,
		customInfo:     option.customInfo,
		additionalJvm:  option.additionalJvm,
		additionalGame: option.additionalGame,
		callback:       callback,
	}
	return ls
}

// checkError 检查参数是否有误
func (lg launchGame) checkError() error {
	if lg.account.GetOnline() == 0 {
		if b, _ := regexp.MatchString("^[a-zA-Z0-9]{3,16}$", lg.account.GetName()); !b {
			return NewMMCLLError(1, "username is invalid")
		}
		if b, _ := regexp.MatchString("^[a-f0-9]{32}$", lg.account.GetUUID()); !b {
			return errors.New("useruuid is invalid")
		}
	} else if lg.account.GetOnline() == 1 {
		//TODO: 微软账户判断
	} else if lg.account.GetOnline() == 2 {
		//TODO: 第三方账号判断
	}
	info, err := os.Stat(lg.javaPath)
	if os.IsNotExist(err) {
		return errors.New("javaPath does not exist")
	} else if info.IsDir() {
		return errors.New("javaPath is a directory")
	}
	info, err = os.Stat(lg.rootPath)
	if os.IsNotExist(err) {
		return errors.New("rootPath does not exist")
	} else if !info.IsDir() {
		return errors.New("rootPath is not a directory")
	}
	info, err = os.Stat(lg.versionPath)
	if os.IsNotExist(err) {
		return errors.New("versionPath does not exist")
	} else if !info.IsDir() {
		return errors.New("versionPath is not a directory")
	}
	info, err = os.Stat(lg.gamePath)
	if os.IsNotExist(err) {
		return errors.New("gamePath does not exist")
	} else if !info.IsDir() {
		return errors.New("gamePath is not a directory")
	}
	sx, sy := robotgo.GetScreenSize()
	if lg.windowWidth < 854 || lg.windowWidth > uint32(sx) {
		return errors.New("window width out of range")
	}
	if lg.windowHeight < 854 || lg.windowHeight > uint32(sy) {
		return errors.New("window height out of range")
	}
	if lg.minMemory < 256 || lg.minMemory > 1024 {
		return errors.New("minMemory out of range")
	}
	v, _ := mem.VirtualMemory()
	sysMem := v.Total / 1024 / 1024
	if lg.maxMemory < 1024 || lg.maxMemory > uint32(sysMem) {
		return errors.New("maxMemory out of range")
	}
	if lg.customInfo == "" {
		return errors.New("customInfo is empty")
	}
	return nil
}

// launch 如果参数无误则尝试启动
func (lg launchGame) launch() error {
	var result []string
	result = append(result, "-XX:+UseG1GC")
	result = append(result, "-XX:-UseAdaptiveSizePolicy")
	result = append(result, "-XX:-OmitStackTraceInFastThrow")
	result = append(result, "-Dfml.ignoreInvalidMinecraftCertificates=true")
	result = append(result, "-Dfml.ignorePatchDiscrepancies=true")
	result = append(result, "-Dlog4j2.formatMsgNoLookups=true")
	// 判断操作系统
	if runtime.GOOS == "windows" {
		result = append(result, "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump")
		if runtime.GOARCH == "386" {
			result = append(result, "-Xss1M")
		}
		if info.GetWindowsVersion() {
			result = append(result, "-Dos.name=Windows 10")
			result = append(result, "-Dos.version=10.0")
		}
	} else if runtime.GOOS == "darwin" {
		result = append(result, "-XstartOnFirstThread")
	}

	return nil
}

// LaunchGame
// 新增参数：isStrict，用于手动指定是否动用 MMCLL 的参数检查。
// 如果你想自己在源代码里检查的话，你完全可以将该值设为 false 以跳过自带的 MMCLL 参数检查。
func LaunchGame(option LaunchOption, isStrict bool, callback func([]string)) error {
	ls := newLaunchStart(option, callback)
	if isStrict {
		if err := ls.checkError(); err != nil {
			return err
		}
	}
	if err := ls.launch(); err != nil {
		return err
	}
	return nil
}
