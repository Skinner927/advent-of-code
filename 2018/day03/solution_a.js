const fabric = [];
const fabricSize = 1000;
// Fill the fabric with zeroes
for (var x = 0; x < fabricSize; x++) {
  for (var y = 0; y < fabricSize; y++) {
    if (!fabric[x]) {
      fabric[x] = [];
    }
    fabric[x][y] = 0;
  }
}

const lineReg = /#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/;

require('fs')
  .readFileSync('data.txt', 'utf8')
  .split('\n')
  .forEach(function(line, i) {
    if (!line) {
      return;
    }

    let match;
    if (!(match = line.match(lineReg))) {
      console.error('skipping line', i, line);
      return;
    }

    let claim = match[1];
    let x = parseInt(match[2], 10);
    let y = parseInt(match[3], 10);
    let w = parseInt(match[4], 10);
    let h = parseInt(match[5], 10);

    for (let j = x; j < x + w; j++) {
      for (let k = y; k < y + h; k++) {
        if (j > fabricSize) {
          console.error('given size too big j', j);
        }
        if (k > fabricSize) {
          console.error('given size too big k', k);
        }
        fabric[j][k]++;
      }
    }
  });

// print it lol
// for (var x = 0; x < 1000; x++) {
//   console.log(fabric[x].reduce((c, v) => c + ',' + v, '')) + '\n';
// }

let overlap = 0;

for (var x = 0; x < fabricSize; x++) {
  for (var y = 0; y < fabricSize; y++) {
    if (fabric[x][y] >= 2) {
      overlap++;
    }
  }
}

console.log('overlap:', overlap);
