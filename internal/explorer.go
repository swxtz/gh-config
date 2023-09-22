package internal

import (
	"os"
	"os/exec"
)

func OpenExplorer() {
	rawDir, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	dir := rawDir + `\fonts\ttf`

	open := exec.Command("explorer", dir)

	open.Run()
}
