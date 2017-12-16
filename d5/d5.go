package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	text, err := ioutil.ReadFile("input.txt")
	if err != nil {
		fmt.Print(err)
	}

	var strInput = strings.Split(string(text), "\n")
	nums := make([]int, len(strInput))
	for i, str := range strInput {
		nums[i], _ = strconv.Atoi(str)
	}

	var steps int
	totSteps := 0
	for i := 0; i < len(strInput) && i >= 0; {
		steps = nums[i]
		nums[i]++
		i += steps
		totSteps++
	}

	fmt.Printf(strconv.Itoa(totSteps))
}
