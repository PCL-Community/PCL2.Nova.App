package mmcll

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
)

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
								if err = json.Unmarshal([]byte(content), &m); err == nil {
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
func FindVanillaPath(versionPath, vanilla string) (string, error) {
	return "", nil
}

// getVanillaVersion 通过JSON获取到原版键值
func GetVanillaVersion(versionJson map[string]interface{}) (string, error) {
	return "", nil
}

func MergeMCJson(jsonContent, realJsonContent map[string]interface{}) (map[string]interface{}, error) {
	return nil, nil
}