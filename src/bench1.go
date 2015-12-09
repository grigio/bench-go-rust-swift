package main
import "fmt"

var iter = 10000000
var sum = 0

var x[]int;
var y[]int;

func main() {

  for i:=0; i<iter; i++ {
    x = append(x, i)
    y = append(y, i + 1)
  }

  for i:=0; i<iter; i++ {
    if i % 5 == 0 && x[i] % 2 == 0 {
      sum += x[i]
    }
  }

  for i:=0; i<iter; i++ {
    if i % 5 == 0 && y[i] % 2 == 0 {
      sum += y[i]
    }
  }

  fmt.Print(sum)
}
