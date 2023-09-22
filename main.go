package main

import (
	// "github.com/fatih/color"
	"log"

	"github.com/fatih/color"
	"github.com/swxtz/gh-config/internal"
)

func main() {
	gitParser, err := internal.Git()
	green := color.New(color.FgGreen)

	if err != nil {
		log.Fatal("gitParser is nil")
	}

	green.Println(gitParser)

	fontDownloader, err := internal.FontDownloader()

	if err != nil {
		log.Fatal("fontDownloader is nil")
	}

	green.Println(fontDownloader)

	unzip, err := internal.Unzip()

	if err != nil {
		log.Fatal(err)
	}

	green.Println(unzip)

	_, err = internal.FontInstaller()

	if err != nil {
		log.Fatal(err)
	}

	internal.OpenExplorer()
}
