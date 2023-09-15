package internal

import "os/exec"

func Git() (string, error) {
	command := "git"

	argsName := []string{"config", "--global", "user.name", "Gustavo Mendonça"}
	argsEmail := []string{"config", "--global", "user.email", "alf4r6@gmail.com"}
	argsDefaultBranch := []string{"config", "--global", "init.defaultBranch", "main"}

	cmdName := exec.Command(command, argsName...)
	cmdEmail := exec.Command(command, argsEmail...)
	cmdDefaultBranch := exec.Command(command, argsDefaultBranch...)

	err := cmdName.Run()

	if err != nil {
		panic(err)
	}

	err = cmdEmail.Run()

	if err != nil {
		panic(err)
	}

	err = cmdDefaultBranch.Run()

	if err != nil {
		panic(err)
	}

	return ("Git config done!"), nil
}
