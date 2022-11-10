import { hello } from './hello.js';

const module = await import(pkg('test'));

const path = `${__dirname}/testing/demo.txt`;

hello();
// test();

console.info('this is a notice');
console.error('this is a error');
console.log(__dirname);
console.log(os.release());
console.log(os.platform());
console.log(os.machine());
console.log(os.uptime());
console.log(os.freemem());
console.log(os.totalmem());
console.log(os.loadavg());

// format demo
console.log(`${os.freemem().parseBytes()}/${os.totalmem().parseBytes()}`);

try {
	const contents = await fs.readFile(path);
	console.log('read', path, '\n' + contents);
} catch (err) {
	console.error('Unable to read file', path, err);
}

process.env.set('DEBUG', 'no');
console.log(process.env.get('DEBUG'));

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
