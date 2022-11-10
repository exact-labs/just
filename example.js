console.log('Hello', 'runjs!');
console.error('Boom!');

const path = './log.txt';
try {
	const contents = await core.readFile(path);
	console.log('Read from a file', contents);
} catch (err) {
	console.error('Unable to read file', path, err);
}

await core.writeFile(path, 'I can write to a file.');
const contents = await core.readFile(path);
console.log('Read from a file', path, 'contents:', contents);
console.log('Removing file', path);
core.removeFile(path);
console.log('File removed');

await new Promise((r) => setTimeout(r, 2000));

console.clear();
