const reg = /(.)(.)/g;

var data = require('fs')
  .readFileSync('data.txt', 'utf8')
  .replace(/[^a-zA-Z]+/g, '');

var m;
while ((m = reg.exec(data)) !== null) {
  let a = m[1];
  let b = m[2];

  //console.log(a, b, ' i:', reg.lastIndex, m.index);
  if (a !== b && a.toLowerCase() === b.toLowerCase()) {
    data = data.replace(a + b, '');
    reg.lastIndex -= 3;
  } else {
    reg.lastIndex -= 1;
  }
}

console.log(data.length);
