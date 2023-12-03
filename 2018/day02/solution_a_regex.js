let has2 = 0;
let has3 = 0;

const regHas2 = /([a-zA-Z])(?<!\1.*\1)(?=(.*\1){1})(?!(.*\1){2})/;
const regHas3 = /([a-zA-Z])(?<!\1.*\1)(?=(.*\1){2})(?!(.*\1){3})/;
// test string: AAzByCCCDxBwDvutDsDDDD

require('fs')
  .readFileSync('data.txt', 'utf8')
  .split('\n')
  .forEach(function(id) {
    if (id.match(regHas2)) {
      has2++;
    }
    if (id.match(regHas3)) {
      has3++;
    }
  });

console.log('checksum:');
console.log(has2 * has3);
