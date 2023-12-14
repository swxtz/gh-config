package font

// https://www.tutorialspoint.com/how-to-rename-and-move-a-file-in-golang#:~:text=In%20Golang%2C%20renaming%20and%20moving,file%20involves%20changing%20its%20path.

import (
	"fmt"
	"os"
)

func FontInstaller() (string, error) {

	destDir := `C:\Windows\Fonts`
	workDir, err := os.Getwd()
	if err != nil {
		return "", err
	}

	sourceDir := workDir + `\fonts\ttf\`

	err = os.Rename(sourceDir+`JetBrainsMono-Regular.ttf`, destDir+`JetBrainsMono-Regular.ttf`)

	if err != nil {
		fmt.Println(err)
	}

	return "Font installed!", nil
}
