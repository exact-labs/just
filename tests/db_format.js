const db = new Database('files/versions');

db.create('versions', 'id text primary key, version text');

await cmd.spawn('rustc -V').then((output) => {
	db.add('versions', { id: core.id.secure(), version: output });
});

console.json(db.get('versions', "where version = '%s'".format(cmd.exec('rustc -V'))), true);

db.rm('versions', '');
