var num = 45;
var fib = function(n) {
  return (n <= 2) ? 1 :
    fib(n - 1) + fib(n - 2);
}

console.log(fib(num));
