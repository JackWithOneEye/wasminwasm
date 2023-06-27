package main

import (
	"fmt"
	"log"
	"syscall/js"
)

func stringLenInfo(this js.Value, args []js.Value) any {
	if len(args) == 0 {
		return "YOU HAVE NOT GIVEN ME ANY ARGUMENTS!!!!"
	}
	if args[0].Type() != js.TypeString {
		return "YOU MUST GIVE ME A STRING ARGUMENT!!!!"
	}
	s := []rune(args[0].String())

	l := len(s)
	var lenInfo string
	if l == 0 {
		lenInfo = "empty"
	} else {
		c := "s"
		if l == 1 {
			c = ""
		}
		lenInfo = fmt.Sprintf("%d character%s long", l, c)
	}
	return fmt.Sprintf("your input is %s", lenInfo)
}

func main() {
	log.Println("HELLO GO!!!!!")
	js.Global().Set("stringLenInfo", js.FuncOf(stringLenInfo))
	<-make(chan bool)
}
