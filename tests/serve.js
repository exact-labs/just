core.print('\x1b[35mcore.uuid() \x1b[0m');
console.log(core.id.uuid());

core.print(`\x1b[35mhttp.get('https://httpbin.org/get') \x1b[0m`);
let json = await http.get('https://httpbin.org/get').then((data) => data.json());

console.log(json.headers['Host']);

server.static(80, './html');
