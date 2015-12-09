package main
import "fmt"

var num = 45

func fib(n int) int {
	if n <= 2 {
		return 1;
	} else {
		return fib(n - 1) + fib(n - 2)
	}
}

func main() {
	fmt.Println(fib(num))
}
