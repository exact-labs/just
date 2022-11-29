package commands

import (
	"fmt"
	"io/ioutil"
)

func Get_file(file string) {
	body, err := ioutil.ReadFile(file)
	if err != nil {
		fmt.Printf("\x1b[31m%v\033[0m", err)
	}
	fmt.Println(string(body))
}
