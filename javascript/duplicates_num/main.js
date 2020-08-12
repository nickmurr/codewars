function duplicateCount(text) {
  let res = {};
  count = 0;
  text
    .toLowerCase()
    .split('')
    .forEach((v) => {
      if (res[v] === 1) {
        count += 1;
      }
      !!res[v] ? (res[v] += 1) : (res[v] = 1);
    });
  return count;
}

duplicateCount('aabBcdeeeeea');
