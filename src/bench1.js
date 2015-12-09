var iter = 10000000;
var sum = 0;

var x = [];
var y = [];

for (var i=0; i<iter; i++) {
  x.push(i)
  y.push(i + 1)
}

for (var i=0; i<iter; i++) {
  if (i % 5 === 0 && x[i] % 2 === 0) {
    sum += x[i];
  }
}

for (var i=0; i<iter; i++) {
  if (i % 5 === 0 && y[i] % 2 === 0) {
    sum += y[i];
  }
}

console.log(sum)
