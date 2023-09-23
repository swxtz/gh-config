package main

import (
	// "github.com/fatih/color"
	"log"

	"github.com/fatih/color"
	"github.com/swxtz/gh-config/internal/downloader"
	"github.com/swxtz/gh-config/internal/explorer"
	"github.com/swxtz/gh-config/internal/font"
	"github.com/swxtz/gh-config/internal/git"
	"github.com/swxtz/gh-config/internal/unzip"
)

func main() {
	gitParser, err := git.Git()
	green := color.New(color.FgGreen)

	if err != nil {
		log.Fatal("gitParser is nil")
	}

	green.Println(gitParser)

	fontDownloader, err := downloader.FontDownloader()

	if err != nil {
		log.Fatal("fontDownloader is nil")
	}

	green.Println(fontDownloader)

	unzip, err := unzip.Unzip()

	if err != nil {
		log.Fatal(err)
	}

	green.Println(unzip)

	_, err = font.FontInstaller()

	if err != nil {
		log.Fatal(err)
	}

	explorer.OpenExplorer()
}
