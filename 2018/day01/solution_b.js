let f = 0;
const pastFreq = {};

const lines = require('fs')
  .readFileSync('data.txt', 'utf8')
  .split('\n');

const ll = lines.length;
let i = -1;
while (true) {
  if (++i >= ll) {
    i = -1;
    continue;
  }
  var str = lines[i];

  if (!str) {
    continue;
  }

  var val = parseInt(str.substring(1), 10)
  if (str[0] === '-') {
    f -= val;
  } else {
    f += val;
  }

  if (f in pastFreq) {
    console.log(f);
    break;
  }
  pastFreq[f] = true;
}
