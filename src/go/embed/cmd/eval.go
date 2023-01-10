package cmd

import (
   "fmt"
   "net/url"
   
	"github.com/spf13/cobra"
	"github.com/traefik/yaegi/interp"
	"github.com/traefik/yaegi/stdlib"
)

var eval_go = &cobra.Command{
	Use:  "eval_go",
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
      eval := interp.New(interp.Options{})
      eval.Use(stdlib.Symbols)
      
      code, err := url.QueryUnescape(args[0])
      if err != nil {
        fmt.Printf("\x1b[31m%v\033[0m", err)
      }
      
      if _, err := eval.Eval(code); err != nil {
        fmt.Printf("\x1b[31m%v\033[0m", err)
      }
	},
}

func init() {
	rootCmd.AddCommand(eval_go)
}
