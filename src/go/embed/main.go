package main

import (
	"errors"
	"flag"
	"reflect"
	"strings"
   
   "external/commands"
)

func Call(m map[string]interface{}, name string, params ...interface{}) (result []reflect.Value, err error) {
	f := reflect.ValueOf(m[name])
	if len(params) != f.Type().NumIn() {
		err = errors.New("The number of params is not adapted.")
		return
	}
	in := make([]reflect.Value, len(params))
	for k, param := range params {
		in[k] = reflect.ValueOf(param)
	}
	result = f.Call(in)
	return
}

func main() {
	funcs := map[string]interface{}{
		"get_file": commands.Get_file,
	}

	value := flag.String("run", "", "")
	flag.Parse()
	args := strings.Split(*value, ":")
	Call(funcs, args[0], args[1])
}


