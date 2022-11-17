const db = new Database('test');

db.create('test', 'id text primary key, name text, age integer');
// db.add('test', { id: core.id.secure(), name: 'hello', age: 1 });

const data = db.get('test', '');

core.print(data[0].id + '\n');

console.log(data);
console.json(data, true);
