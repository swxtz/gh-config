package downloader

import (
	"io"
	"net/http"
	"os"
)

// font link: https://www.jetbrains.com/lp/mono/#how-to-install
//https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip?_gl=1*7ies17*_ga*MTM3NjY4MDIwMy4xNjk0ODE2NTYw*_ga_9J976DJZ68*MTY5NDgxOTUxMC4yLjAuMTY5NDgxOTUxMS41OS4wLjA.&_ga=2.256821067.1940918658.1694816560-1376680203.1694816560

func FontDownloader() (string, error) {
	url := "https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip?_gl=1*7ies17*_ga*MTM3NjY4MDIwMy4xNjk0ODE2NTYw*_ga_9J976DJZ68*MTY5NDgxOTUxMC4yLjAuMTY5NDgxOTUxMS41OS4wLjA.&_ga=2.256821067.1940918658.1694816560-1376680203.1694816560"

	res, err := http.Get(url)

	if err != nil {
		return "", err
	}

	defer res.Body.Close()

	if res.StatusCode != http.StatusOK {
		panic("status code error: " + res.Status)
	}

	out, err := os.Create("JetBrainsMono.zip")

	if err != nil {
		return "", err
	}

	defer out.Close()

	_, err = io.Copy(out, res.Body)

	if err != nil {
		return "", err
	}

	return "Font downloaded!", nil

}
