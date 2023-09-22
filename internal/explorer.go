package internal

import (
	"os"
)

func OpenExplorer() {
	rawDir, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	dir := rawDir + `\fonts\ttf`

	println(dir)
}
