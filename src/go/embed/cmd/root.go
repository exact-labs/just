package cmd

import (
   "fmt"
   "os"
   "os/signal"

   "github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
   Use:     "external",
   Version: "v0.4.3",
}

func Execute() {
   c := make(chan os.Signal, 1)
   signal.Notify(c, os.Interrupt)
   go func() {
      <-c
      fmt.Println("user interrupt")
      os.Exit(0)
   }()

   if err := rootCmd.Execute(); err != nil {
      os.Exit(1)
   }
}