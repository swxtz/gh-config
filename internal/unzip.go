package internal

import (
	"archive/zip"
	"io"
	"os"

	"github.com/fatih/color"
)

func Unzip() (string, error) {
	zipFile := "JetBrainsMono.zip"
	yellow := color.New(color.FgYellow)

	r, err := zip.OpenReader(zipFile)

	if err != nil {
		panic(err)
	}

	defer r.Close()

	for _, f := range r.File {
		yellow.Println("Extraindo \n: ", f.Name)

		rc, err := f.Open()

		if err != nil {
			panic(err)
		}

		defer rc.Close()

		dstFile, err := os.Create(f.Name)

		if err != nil {
			panic(err)
		}

		defer dstFile.Close()

		_, err = io.Copy(dstFile, rc)
		if err != nil {
			panic(err)
		}

	}

	return "Fonte extraida!", nil
}
