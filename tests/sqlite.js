const db = new Database('test');

db.create('test', 'id text primary key, name text, age integer');
// db.add('test', { id: core.id.secure(), name: 'hello', age: 1 });

// core.print(db.get('test', ''));
db.get('test', '');
