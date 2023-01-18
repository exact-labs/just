package cmd

import (
	"fmt"
	"io/ioutil"

	"github.com/spf13/cobra"
)

var get_file = &cobra.Command{
	Use:   "get_file",
	Args:  cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		file, _ := cmd.Flags().GetString("path")
		body, err := ioutil.ReadFile(file)
      
		if err != nil {
			fmt.Printf("\x1b[31m%v\033[0m", err)
		}
      
		fmt.Println(string(body))
	},
}

func init() {
	rootCmd.AddCommand(get_file)

	get_file.Flags().StringP("path", "p", "", "")
	get_file.MarkFlagRequired("path")
}
