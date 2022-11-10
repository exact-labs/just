import { hello } from './hello.js';

const path = `${__dirname}/testing/demo.txt`;

hello();

console.info('this is a notice');
console.error('this is a error');
console.log(__dirname);
console.log(os.release());
console.log(os.platform());
console.log(os.freemem());
console.log(os.totalmem());

// format demo
console.log(`${os.freemem().parseBytes()}/${os.totalmem().parseBytes()}`);

try {
	const contents = await core.readFile(path);
	console.log('read', path, '\n' + contents);
} catch (err) {
	console.error('Unable to read file', path, err);
}

console.log(process.env.get('DEBUG'));
console.log(process.env.toObject());

// if (process.env.get('DEBUG')) {
// 	// writing
// 	await core.writeFile(path, 'Hello World');
// 	const contents = await core.readFile(path);
// 	console.log('read:', path, 'contents:', contents);
//
// 	// removing
// 	console.log('Removing file', path);
// 	core.removeFile(path);
// 	console.log('File removed');
//
// 	// sleep
// 	await sleep('2s');
// 	console.clear();
// }
