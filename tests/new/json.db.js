import { JSONdb } from 'https://r.justjs.dev/jsondb';
import { fs } from 'just/io';

const db = await new JSONdb('storage.json');

db.set('json', { hello: 'world' });
console.log(db.get('json'));
console.log(db.has('json'));

db.set('number', 500);
console.log(db.get('number'));
console.log(db.has('number'));

db.set('arr', [1, 2, 3, 4, 5]);
console.log(db.get('arr'));
console.log(db.has('arr'));

console.log(db.JSON());

db.delete('json');
db.delete('arr');
db.delete('number');

await db.sync().then(() => {
	fs.file.remove(__dirname + '/storage.json');
});
