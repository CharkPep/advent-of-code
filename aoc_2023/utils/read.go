package utils

import (
	"os"
	"path/filepath"
)

func ReadFile(path string) string {
	absPath, err := filepath.Abs(path)
	text, err := os.ReadFile(absPath)
	if err != nil {
		panic(err)
	}

	return string(text)
}
