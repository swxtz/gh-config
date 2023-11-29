package unzip

import (
	"archive/zip"
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/fatih/color"
)

func Unzip() (string, error) {
	zipFile := "JetBrainsMono.zip"
	yellow := color.New(color.FgYellow)

	r, err := zip.OpenReader(zipFile) // r é o array

	if err != nil {
		return "", fmt.Errorf("erro abrindo zip")
	}

	defer r.Close()

	err = os.MkdirAll("fonts/ttf/", 0777)

	if err != nil {
		return "", err
	}

	for _, f := range r.File {
		if strings.HasPrefix(f.Name, "fonts/ttf/") && !f.FileInfo().IsDir() {
			yellow.Println("Extraindo : ", f.Name)

			rc, err := f.Open()

			if err != nil {
				return "", fmt.Errorf("error while opening")
			}

			defer rc.Close()

			dstFile, err := os.Create(f.Name)

			if err != nil {
				return "", fmt.Errorf("erro criando arquivo")
			}

			defer dstFile.Close()

			_, err = io.Copy(dstFile, rc)

			if err != nil {
				return "", fmt.Errorf("erro copiando conteúdo")
			}

		}
	}

	return "Fonte extraida!", nil
}
