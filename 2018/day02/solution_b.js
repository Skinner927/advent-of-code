let has2 = 0;
let has3 = 0;
const boxes = [];

const regHas2 = /([a-zA-Z])(?<!\1.*\1)(?=(.*\1){1})(?!(.*\1){2})/;
const regHas3 = /([a-zA-Z])(?<!\1.*\1)(?=(.*\1){2})(?!(.*\1){3})/;
// test string: AAzByCCCDxBwDvutDsDDDD

require('fs')
  .readFileSync('data.txt', 'utf8')
  .split('\n')
  .forEach(function(id) {
    if (id.match(regHas2)) {
      has2++;
      boxes.push(id);
    } else if (id.match(regHas3)) {
      has3++;
      boxes.push(id);
    }
  });

console.log('checksum:');
console.log(has2 * has3);

// This is super brute force
var shortest = Number.MAX_SAFE_INTEGER;
var a = null;
var b = null;

function diff(a, b) {
  // Count missing
  return a.split('').reduce(function(count, c, i) {
    if (b[i] !== c) {
      return count + 1;
    }
    return count;
  }, 0);
}

for (var i = 0; i < boxes.length; i++) {
  for (var j = 0; j < boxes.length; j++) {
    if (i === j) {
      // Do not check ourselves
      continue;
    }
    var missing = diff(boxes[i], boxes[j]);
    if (missing < shortest) {
      shortest = missing;
      a = boxes[i];
      b = boxes[j];
    }
  }
}

var common = a.split('').reduce(function(agg, c, i) {
  if (b[i] === c) {
    return agg + c;
  }
  return agg;
}, '');

console.log('shortest', shortest, a, b, 'common: ', common);

