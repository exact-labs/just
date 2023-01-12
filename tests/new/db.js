import { cmd } from 'just/sys';
import { random } from 'just/crypto';
import { Database } from 'just/db:sqlite';

const db = new Database('versions');

db.create('versions', 'id text primary key, version text');
await cmd.spawn('rustc -V').then((output) => {
	db.insert('versions', { id: random.secure(), version: output });
});

console.json(db.query('versions', "where version = '%s'".format(cmd.exec('rustc -V'))), true);

db.remove('versions', '');
