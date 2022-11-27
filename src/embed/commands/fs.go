package commands

import (
	"fmt"
	"io/ioutil"
	"log"
)

func Get_file(file string) {
	body, err := ioutil.ReadFile(file)
	if err != nil {
		log.Fatalf("unable to read file: %v", err)
	}
	fmt.Print(string(body))
}
