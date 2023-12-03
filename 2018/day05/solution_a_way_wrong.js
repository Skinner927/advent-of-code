/**
 * I read the directions wrong. This gets rid of elements even if they're
 * not touching.
 */

var map = {};

function add(map, key, val) {
  key = key.toLowerCase();
  if (!(key in map)) {
    map[key] = 0;
  }
  map[key] += val;
}

require('fs')
  .readFileSync('data.txt', 'utf8')
  .split('')
  .forEach(function(c) {
    if (!c) { return; }
    var ccode = c.charCodeAt(0);
    console.log(ccode);

    if (ccode >= 65 && ccode <= 90) {
      add(map, c, 1);
    } else if (ccode >= 97) {
      add(map, c, -1);
    }
  });

var count = Object.keys(map).filter(function(key) {
  console.log(key, map[key], typeof map[key]);
  return map[key] !== 0;
});

console.log(count.length);
