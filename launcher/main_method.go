package launcher

import (
	"NovaImitation/info"
	"NovaImitation/mmcll"
	"io/fs"
	"math"
	"os"
	"path/filepath"
	"regexp"
)

type ReaderWriter struct{}
type MainMethod struct{}

func EnsureConfigFile(path string) error {
	if _, err := os.Stat(path); err == nil {
		return nil // 文件已存在
	}
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, fs.ModePerm); err != nil {
		return err
	}
	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()
	return nil
}

func (rw *ReaderWriter) WriteConfig(path, section, key, value string) {
	conf := NewConfig(path)
	err := conf.Write(section, key, value)
	if err != nil {
		panic(err)
	}
}
func (rw *ReaderWriter) ReadConfig(path, section, key string) string {
	conf := NewConfig(path)
	if v, err := conf.Read(section, key); err == nil {
		return v
	}
	return ""
}
func (rw *ReaderWriter) GetOtherIniPath() string {
	home, err := info.GetHomeDir()
	if err != nil {
		exePath, err := os.Executable()
		if err != nil {
			panic(err)
		}
		home = filepath.Join(filepath.Dir(exePath), "PCL.Nova", "config")
	}
	res := filepath.Join(home, "Other.ini")
	err = EnsureConfigFile(res)
	if err != nil {
		panic(err)
	}
	return res
}
func (rw *ReaderWriter) GetConfigIniPath() string {
	exePath, err := os.Executable()
	if err != nil {
		panic(err)
	}
	res := filepath.Join(filepath.Dir(exePath), "PCL.Nova", "config", "PCL2.Nova.ini")
	err = EnsureConfigFile(res)
	if err != nil {
		panic(err)
	}
	return res
}

func (mm *MainMethod) GenerateBukkitUUID(username string) string {
	return mmcll.GenerateBukkitUUID(username)
}

func (mm *MainMethod) UUIDToAvatar(uuid string) int64 {
	regex, _ := regexp.Compile("^[a-z0-9]{32}$")
	if regex.MatchString(uuid) {
		bin := ""
		for _, char := range uuid {
			switch char {
			case '0':
				bin += "0000"
				break
			case '1':
				bin += "0001"
				break
			case '2':
				bin += "0010"
				break
			case '3':
				bin += "0011"
				break
			case '4':
				bin += "0100"
				break
			case '5':
				bin += "0101"
				break
			case '6':
				bin += "0110"
				break
			case '7':
				bin += "0111"
				break
			case '8':
				bin += "1000"
				break
			case '9':
				bin += "1001"
				break
			case 'a':
				bin += "1010"
				break
			case 'b':
				bin += "1011"
				break
			case 'c':
				bin += "1100"
				break
			case 'd':
				bin += "1101"
				break
			case 'e':
				bin += "1110"
				break
			case 'f':
				bin += "1111"
				break
			default:
				panic("[Invalid Input!]")
			}
		}
		most1 := bin[0:64]
		least1 := bin[64:128]
		xor1 := ""
		for index := 0; index < 64; index++ {
			if most1[index] == least1[index] {
				xor1 += "0"
			} else {
				xor1 += "1"
			}
		}
		most2 := xor1[0:32]
		least2 := xor1[32:64]
		xor2 := ""
		for index := 0; index < 32; index++ {
			if most2[index] == least2[index] {
				xor2 += "0"
			} else {
				xor2 += "1"
			}
		}
		var ten int64
		for index := 0; index < 32; index++ {
			if xor2[index] == '1' {
				ten += int64(math.Trunc(math.Pow(float64(len(xor2)-index), 2.0)))
			}
		}
		return ten
	}
	return -1
}
