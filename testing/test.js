core.print('\x1b[35mcore.uuid() \x1b[0m');
console.log(core.uuid());

core.print(`\x1b[35mhttp.get('https://api.github.com/users/theMackabu/repos') \x1b[0m`);
let json = await http.get('https://httpbin.org/get').then((data) => data.json());

console.log(json.headers['Host']);
