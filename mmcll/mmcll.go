package mmcll

import (
	"NovaImitation/info"
	"crypto/sha1"
	"encoding/json"
	"fmt"
	"github.com/go-vgo/robotgo"
	"github.com/shirou/gopsutil/mem"
	"io"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"strings"
)

// SomeConst.go

const (
	LauncherName           = "MMCLL"          // LauncherName 启动器名称（请自行修改）
	LauncherVersion        = "0.0.1-Alpha-12" // LauncherVersion 启动器版本
	LauncherUserAgent      = "MMCLL/0.0.1.12" // LauncherUserAgent 启动器UserAgent，用于在网络请求时的设置
	ErrUserNameInvalid     = 1                // ErrUserNameInvalid 启动游戏时用户名输入不正确
	ErrUserUUIDInvalid     = 2                // ErrUserUUIDInvalid 启动游戏时用户UUID输入不正确
	ErrJavaPathInvalid     = 3                // ErrJavaPathInvalid Java路径错误
	ErrRootPathInvalid     = 4                // ErrRootPathInvalid 游戏根路径错误
	ErrVersionPathInvalid  = 5                // ErrVersionPathInvalid 游戏根路径错误
	ErrGamePathInvalid     = 6                // ErrGamePathInvalid 游戏根路径错误
	ErrWidthOutOfRange     = 7                // ErrWidthOutOfRange 窗口宽度超出范围
	ErrHeightOutOfRange    = 8                // ErrHeightOutOfRange 窗口高度超出范围
	ErrMinMemoryOutOfRange = 9                // ErrMinMemoryOutOfRange 最小内存超出范围
	ErrMaxMemoryOutOfRange = 10               // ErrMaxMemoryOutOfRange 最大内存超出范围
	ErrCustomInfoIsEmpty   = 11               // ErrCustomInfoIsEmpty 自定义信息为空
)

// ErrorMMCLL 定义报错类型
type ErrorMMCLL struct {
	code int32  // code 报错代码
	msg  string // msg 报错信息
}

func (e ErrorMMCLL) Error() string {
	return fmt.Sprintf("Err Code: %d, Err Message: %s", e.code, e.msg)
}
func NewMMCLLError(code int32, msg string) ErrorMMCLL {
	return ErrorMMCLL{code, msg}
}

// MainMethod.go

// GetFile 获取文件内容
func GetFile(path string) (string, error) {
	open, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer open.Close()
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

// LaunchMethod.go

// GetMCRealPath 用来获取MC的文件绝对路径。（可以通过后缀或者sha1来判断）
func GetMCRealPath(versionPath, suffix string) (string, error) {
	path, err := os.Stat(versionPath)
	if err == nil && path.IsDir() {
		dir, err2 := os.Open(versionPath)
		if err2 == nil {
			defer dir.Close()
			files, err3 := dir.Readdir(-1)
			if err3 == nil {
				for _, file := range files {
					if file.IsDir() {
						continue
					}
					name := file.Name()
					path2 := filepath.Join(versionPath, name)
					if strings.Contains(name, suffix) {
						if suffix == ".json" && name[len(name)-5:] == ".json" {
							content, err4 := GetFile(path2)
							if err4 == nil {
								var m map[string]interface{}
								if err := json.Unmarshal([]byte(content), &m); err == nil {
									continue
								}
								if _, ok1 := m["libraries"].([]interface{}); !ok1 {
									continue
								}
								if _, ok2 := m["mainClass"].(string); !ok2 {
									continue
								}
								if _, ok3 := m["id"].(string); !ok3 {
									continue
								}
								return path2, nil
							}
						} else {
							return path2, nil
						}
					} else if !strings.Contains(suffix, ".") {
						if sha, err4 := GetSha1(path2); err4 == nil {
							if sha == suffix {
								return path2, nil
							}
						}
					}
				}
				err = NewMMCLLError(201, "Cannot find "+suffix+" file in path "+versionPath+".")
			}
			err = err3
		}
		err = err2
	}
	return "", err
}

// findVanillaPath 通过原版键值准确找到原版游戏路径
func findVanillaPath(versionPath, vanilla string) (string, error) {
	return "", nil
}

// getVanillaVersion 通过JSON获取到原版键值
func getVanillaVersion(versionJson map[string]interface{}) (string, error) {
	return "", nil
}

// launchAccount 用来登录账号所需要的所有键，仅能new一次，后续无法修改。
type launchAccount struct {
	name        string // name 账户名字
	uuid        string // uuid 账号UUID
	accessToken string // accessToken 正版账号登录密钥
	atype       string // atype 账号登录类型
	base        string // base 账号登录第三方数据（仅限外置登录）
	url         string // url 账号登录第三方网址（仅限外置登录）
	online      int8   // online 账号类型（1：离线、2：正版、3：外置）
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

// launchOption 启动设置类（新建后无法修改account、javaPath、rootPath、versionPath几个必填项。）
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

// NewLaunchOption 新建一个启动设置类。（以下非必填的可以直接链式调用设置初始值）
func NewLaunchOption(account launchAccount, javaPath, rootPath, versionPath, gamePath string) *launchOption {
	return &launchOption{
		Account:        account,
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
func (opt *launchOption) SetWindowWidth(windowWidth uint32) *launchOption {
	opt.windowWidth = windowWidth
	return opt
}
func (opt *launchOption) SetWindowHeight(windowHeight uint32) *launchOption {
	opt.windowHeight = windowHeight
	return opt
}
func (opt *launchOption) SetMinMemory(minMemory uint32) *launchOption {
	opt.minMemory = minMemory
	return opt
}
func (opt *launchOption) SetMaxMemory(maxMemory uint32) *launchOption {
	opt.maxMemory = maxMemory
	return opt
}
func (opt *launchOption) SetCustomInfo(customInfo string) *launchOption {
	opt.customInfo = customInfo
	return opt
}
func (opt *launchOption) SetAdditionalJvm(additionalJvm string) *launchOption {
	opt.additionalJvm = additionalJvm
	return opt
}
func (opt *launchOption) SetAdditionalGame(additionalGame string) *launchOption {
	opt.additionalGame = additionalGame
	return opt
}
func (opt *launchOption) GetAccount() launchAccount {
	return opt.Account
}
func (opt *launchOption) GetJavaPath() string {
	return opt.javaPath
}
func (opt *launchOption) GetRootPath() string {
	return opt.rootPath
}
func (opt *launchOption) GetVersionPath() string {
	return opt.versionPath
}
func (opt *launchOption) GetGamePath() string {
	return opt.gamePath
}
func (opt *launchOption) GetWindowHeight() uint32 {
	return opt.windowHeight
}
func (opt *launchOption) GetWindowWidth() uint32 {
	return opt.windowWidth
}
func (opt *launchOption) GetMinMemory() uint32 {
	return opt.minMemory
}
func (opt *launchOption) GetMaxMemory() uint32 {
	return opt.maxMemory
}
func (opt *launchOption) GetCustomInfo() string {
	return opt.customInfo
}
func (opt *launchOption) GetAdditionalJvm() string {
	return opt.additionalJvm
}
func (opt *launchOption) GetAdditionalGame() string {
	return opt.additionalGame
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

// NewLaunchStart 初始化启动类
func newLaunchStart(option launchOption, callback func([]string)) launchGame {
	ls := launchGame{
		account:        option.Account,
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
			return NewMMCLLError(ErrUserNameInvalid, "username is invalid")
		}
		if b, _ := regexp.MatchString("^[a-f0-9]{32}$", lg.account.GetUUID()); !b {
			return NewMMCLLError(ErrUserUUIDInvalid, "useruuid is invalid")
		}
	} else if lg.account.GetOnline() == 1 {
		//TODO: 微软账户判断
	} else if lg.account.GetOnline() == 2 {
		//TODO: 第三方账号判断
	}
	cInfo, err := os.Stat(lg.javaPath)
	if os.IsNotExist(err) {
		return NewMMCLLError(ErrJavaPathInvalid, "javaPath does not exist")
	} else if cInfo.IsDir() {
		return NewMMCLLError(ErrJavaPathInvalid, "javaPath is a directory")
	}
	cInfo, err = os.Stat(lg.rootPath)
	if os.IsNotExist(err) {
		return NewMMCLLError(ErrRootPathInvalid, "rootPath does not exist")
	} else if !cInfo.IsDir() {
		return NewMMCLLError(ErrRootPathInvalid, "rootPath is not a directory")
	}
	cInfo, err = os.Stat(lg.versionPath)
	if os.IsNotExist(err) {
		return NewMMCLLError(ErrVersionPathInvalid, "versionPath does not exist")
	} else if !cInfo.IsDir() {
		return NewMMCLLError(ErrVersionPathInvalid, "versionPath is not a directory")
	}
	cInfo, err = os.Stat(lg.gamePath)
	if os.IsNotExist(err) {
		return NewMMCLLError(ErrGamePathInvalid, "gamePath does not exist")
	} else if !cInfo.IsDir() {
		return NewMMCLLError(ErrGamePathInvalid, "gamePath is not a directory")
	}
	sx, sy := robotgo.GetScreenSize()
	if lg.windowWidth < 854 || lg.windowWidth > uint32(sx) {
		return NewMMCLLError(ErrWidthOutOfRange, "window width out of range")
	}
	if lg.windowHeight < 480 || lg.windowHeight > uint32(sy) {
		return NewMMCLLError(ErrHeightOutOfRange, "window height out of range")
	}
	if lg.minMemory < 256 || lg.minMemory > 1024 {
		return NewMMCLLError(ErrMinMemoryOutOfRange, "minMemory out of range")
	}
	v, _ := mem.VirtualMemory()
	sysMem := v.Total / 1024 / 1024
	if lg.maxMemory < 1024 || lg.maxMemory > uint32(sysMem) {
		return NewMMCLLError(ErrMaxMemoryOutOfRange, "maxMemory out of range")
	}
	if lg.customInfo == "" {
		return NewMMCLLError(ErrCustomInfoIsEmpty, "customInfo is empty")
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
	jsonPath, err := GetMCRealPath(lg.versionPath, ".json")
	if err != nil || jsonPath == "" {
		return err
	}
	jsonContent, err := GetFile(jsonPath)
	if err != nil {
		return err
	}
	var jsonStruct map[string]interface{}
	if err = json.Unmarshal([]byte(jsonContent), &jsonStruct); err != nil {
		return err
	}
	//var inheritsJson string
	//inheritsFrom, ok1 := jsonStruct["inheritsFrom"].(string)
	//if ok1 {
	//	vanillaPath, err := findVanillaPath(lg.versionPath, inheritsFrom)
	//	if err != nil || vanillaPath == "" {
	//		return err
	//	}
	//} else {
	//	//inheritsJson = jsonPath
	//}
	return nil
}

// LaunchGame
// 新增参数：isStrict，用于手动指定是否动用 MMCLL 的参数检查。
// 如果你想自己在源代码里检查的话，你完全可以将该值设为 false 以跳过自带的 MMCLL 参数检查。
func LaunchGame(option launchOption, isStrict bool, callback func([]string)) error {
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
