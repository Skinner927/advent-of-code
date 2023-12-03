const text = require('fs')
  .readFileSync('data.txt', 'utf8')
  .replace(/[^a-zA-Z]+/g, '');

/**
 * Given a string, run all reactions in it.
 * @param {String} data Reaction string
 * @param {RegExp} [reg] By default runs all reactions, otherwise only
 *  replace reactions that match the given regex.
 */
function react(data) {
  const reg = /(.)(.)/g;
  let m = null;
  while ((m = reg.exec(data)) !== null) {
    let a = m[1];
    let b = m[2];

    //console.log(a, b, ' i:', reg.lastIndex, m.index);
    if (a !== b && a.toLowerCase() === b.toLowerCase()) {
      data = data.replace(new RegExp(`(${a}${b}|${b}${a})`, 'g'), '');
      reg.lastIndex -= 3;
    } else {
      reg.lastIndex -= 1;
    }
  }
  return data;
}

let lowest = Number.MAX_SAFE_INTEGER;
for (let i = 0; i < 26; i++) {
  // Create regexp we can use to remove all of one type of element
  let upper = String.fromCharCode(i + 65);
  let lower = String.fromCharCode(i + 97);
  let reg = new RegExp('[' + upper + lower + ']', 'g');

  let removed = text.replace(reg, '').replace(reg, '');
  let reacted = react(removed);
  console.log(upper, reacted.length);
  // Figure out if we're the lowest
  lowest = Math.min(lowest, reacted.length);
}

console.log('Reaction chain:', react(text).length);
console.log('With one removed:', lowest);
