package internal

import (
	"errors"
	"os/exec"
)

func Winget(pkg string, verb string) error {
	command := exec.Command("winget", verb, pkg)
	return command.Run()

	exec := exec.Command(command, verb, pkg)

	if exec != nil {
		println(exec)
		return errors.New("error")
	}

	return nil
}
