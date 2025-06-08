package mmcll

import (
	"NovaImitation/core/utils"
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"runtime"

	"github.com/go-vgo/robotgo"
	"github.com/shirou/gopsutil/mem"
)

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
		if utils.GetWindowsVersion() {
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
	var inheritsJson map[string]interface{}
	inheritsFrom, ok1 := jsonStruct["inheritsFrom"].(string)
	if ok1 {
		vanillaPath, err := FindVanillaPath(lg.versionPath, inheritsFrom)
		if err != nil || vanillaPath == "" {
			return err
		}
		realJsonPath, err := GetMCRealPath(vanillaPath, ".json")
		if err != nil || realJsonPath == "" {
			return err
		}
		realJsonContent, err := GetFile(realJsonPath)
		if err != nil {
			return err
		}
		var realJsonStruct map[string]interface{}
		if err = json.Unmarshal([]byte(realJsonContent), &realJsonStruct); err != nil {
			return err
		}
		inheritsJson, err = MergeMCJson(jsonStruct, realJsonStruct)
		if err != nil || inheritsJson == nil {
			return err
		}
	} else {
		inheritsJson = jsonStruct
	}
	fmt.Println(inheritsJson)
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
