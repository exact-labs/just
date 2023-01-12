const db = new Database('files/test');

db.create('test', 'id text primary key, name text, some_int integer');

db.add('test', { id: core.id.secure(), name: 'hello', some_int: 1 });
db.add('test', { id: core.id.secure(), name: 'world', some_int: 2 });

const data = db.get('test', '');

core.print(data[0].id + '\n');
console.log(data);
console.json(data, true);

db.rm('test', '');
