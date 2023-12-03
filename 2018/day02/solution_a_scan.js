const data = require('fs').readFileSync('data.txt', 'utf8').split('\n');

let has2 = 0;
let has3 = 0;

data.forEach(function(id) {
  let map = id.split('').reduce(function(map, cc) {
    let count = map[cc] || 0;
    map[cc] = ++count;
    return map;
  }, {});

  const appears2 = [];
  const appears3 = [];
  Object.keys(map).forEach(function(key) {
    switch (map[key]) {
      case 2:
        appears2.push(key)
        break;
      case 3:
        appears3.push(key)
        break;
    }
  });

  if (appears2.length > 0) {
    has2++;
  }
  if (appears3.length > 0) {
    has3++;
  }
});

console.log('checksum:');
console.log(has2 * has3);
