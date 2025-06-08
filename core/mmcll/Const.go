package mmcll;

const (
	LauncherName      = "MMCLL"          // LauncherName 启动器名称（请自行修改）
	LauncherVersion   = "0.0.1-Alpha-12" // LauncherVersion 启动器版本
	LauncherUserAgent = "MMCLL/0.0.1.12" // LauncherUserAgent 启动器UserAgent，用于在网络请求时的设置
	// 启动游戏检查
	ErrUserNameInvalid     = 1  // ErrUserNameInvalid 启动游戏时用户名输入不正确
	ErrUserUUIDInvalid     = 2  // ErrUserUUIDInvalid 启动游戏时用户UUID输入不正确
	ErrJavaPathInvalid     = 3  // ErrJavaPathInvalid Java路径错误
	ErrRootPathInvalid     = 4  // ErrRootPathInvalid 游戏根路径错误
	ErrVersionPathInvalid  = 5  // ErrVersionPathInvalid 游戏根路径错误
	ErrGamePathInvalid     = 6  // ErrGamePathInvalid 游戏根路径错误
	ErrWidthOutOfRange     = 7  // ErrWidthOutOfRange 窗口宽度超出范围
	ErrHeightOutOfRange    = 8  // ErrHeightOutOfRange 窗口高度超出范围
	ErrMinMemoryOutOfRange = 9  // ErrMinMemoryOutOfRange 最小内存超出范围
	ErrMaxMemoryOutOfRange = 10 // ErrMaxMemoryOutOfRange 最大内存超出范围
	ErrCustomInfoIsEmpty   = 11 // ErrCustomInfoIsEmpty 自定义信息为空
)
