package main

import (
	"github.com/fatih/color"
	"github.com/swxtz/gh-config/internal"
)

func main() {
	gitParser, err := internal.Git()
	green := color.New(color.FgGreen)

	if err != nil {
		panic("gitParser is nil")
	}

	green.Println(gitParser)

	fontDownloader, err := internal.FontDownloader()

	if err != nil {
		panic("fontDownloader is nil")
	}

	green.Println(fontDownloader)

	unzip, err := internal.Unzip()

	if err != nil {
		panic("unzip is nil")
	}

	green.Println(unzip)

}
