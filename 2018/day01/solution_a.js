var f = 0;

require('fs')
  .readFileSync('data.txt', 'utf8')
  .split('\n')
  .forEach(function(str) {
    if (!str) {
      return;
    }
    var val = parseInt(str.substring(1), 10)
    if (str[0] === '-') {
      f -= val;
    } else {
      f += val;
    }
  });

console.log(f);
