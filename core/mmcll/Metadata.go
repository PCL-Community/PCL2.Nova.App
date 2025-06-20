package mmcll

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
