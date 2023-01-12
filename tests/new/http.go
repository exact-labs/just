package main

import (
   "fmt"
   "io"
   "net/http"
)

func main() {
   http.HandleFunc("/hello", func(w http.ResponseWriter, r *http.Request) {
      fmt.Println("got /hello request\n")
      io.WriteString(w, "Hello, HTTP!\n")
   })

   http.ListenAndServe(":8080", nil)
}
