import { hello } from './hello.js';

const { test } = await import(pkg('test'));
const path = `${__dirname}/testing/demo.txt`;

core.print('\x1b[35mconsole.log (import) \x1b[0m');
hello();
core.print('\x1b[35mconsole.log (pkg) \x1b[0m');
test();

core.print('\x1b[35mconsole.info \x1b[0m');
console.info('this is a notice');
core.print('\x1b[35mconsole.error \x1b[0m');
console.error('this is a error');
core.print('\x1b[35m__dirname \x1b[0m');
console.log(__dirname);
core.print('\x1b[35mos.release() \x1b[0m');
console.log(os.release());
core.print('\x1b[35mos.platform() \x1b[0m');
console.log(os.platform());
core.print('\x1b[35mos.homedir() \x1b[0m');
console.log(os.homedir());
core.print('\x1b[35mos.hostname() \x1b[0m');
console.log(os.hostname());
core.print('\x1b[35mos.machine() \x1b[0m');
console.log(os.machine());
core.print('\x1b[35mos.cpus() \x1b[0m');
console.log(os.cpus());
core.print('\x1b[35mos.uptime() \x1b[0m');
console.log(os.uptime());
core.print('\x1b[35mos.freemem() \x1b[0m');
console.log(os.freemem());
core.print('\x1b[35mos.totalmem() \x1b[0m');
console.log(os.totalmem());
core.print('\x1b[35mos.loadavg() \x1b[0m');
console.log(os.loadavg());

// format demo
core.print('\x1b[35m.parseBytes() \x1b[0m');
console.log(`${os.freemem().parseBytes()}/${os.totalmem().parseBytes()}`);

core.print('\x1b[35mreadFile() \x1b[0m');
try {
	const contents = await fs.readFile(path);
	core.print('\x1b[35mfile contents: \x1b[0m');
	console.log('read', path, '\n' + contents);
} catch (err) {
	console.error('Unable to read file', path, err);
}
core.print('\x1b[35mset/view env \x1b[0m');
process.env.set('DEBUG', 'no');
console.log(process.env.get('DEBUG'));

core.print(`\x1b[35mhttp.get('https://httpbin.org/json') \x1b[0m`);
http.get('https://httpbin.org/json').then((data) => console.log(data));

if (process.env.get('DEBUG') == 'yes') {
	// writing
	await fs.writeFile(path, 'Hello World');
	const contents = await fs.readFile(path);
	console.log('read:', path, 'contents:', contents);

	// removing
	console.log('Removing file', path);
	fs.removeFile(path);
	console.log('File removed');

	// sleep
	await sleep('2s');
	console.clear();
}
