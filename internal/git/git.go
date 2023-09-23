package git

import "os/exec"

func Git() (string, error) {
	command := "git"

	argsName := []string{"config", "--global", "user.name", "Gustavo Mendonça"}
	argsEmail := []string{"config", "--global", "user.email", "alf4r6@gmail.com"}
	argsDefaultBranch := []string{"config", "--global", "init.defaultBranch", "main"}
	argsFinalLines := []string{"config", "--global", "core.autocrlf", "false"}

	cmdName := exec.Command(command, argsName...)
	cmdEmail := exec.Command(command, argsEmail...)
	cmdDefaultBranch := exec.Command(command, argsDefaultBranch...)
	cmdFinalLines := exec.Command(command, argsFinalLines...)

	err := cmdName.Run()

	if err != nil {
		return "", err
	}

	err = cmdEmail.Run()

	if err != nil {
		return "", err
	}

	err = cmdDefaultBranch.Run()

	if err != nil {
		return "", err
	}

	err = cmdFinalLines.Run()

	if err != nil {
		return "", err
	}

	return "Git config done!", nil
}
