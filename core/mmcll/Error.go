package mmcll

import "fmt"

func (e errorMMCLL) Error() string {
	return fmt.Sprintf("Err Code: %d, Err Message: %s", e.code, e.msg)
}

func NewMMCLLError(code int32, msg string) errorMMCLL {
	return errorMMCLL{code, msg}
}
