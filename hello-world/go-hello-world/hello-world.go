package main

import (
	"fmt"
	"os"
)

func main() {
	var name string
	if len(os.Args) > 1 {
		name = os.Args[1]
	} else {
		name = "Nobody"
	}
	fmt.Printf("Hello %v\n", name)
}
