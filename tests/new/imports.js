import { Database } from 'https://r.justjs.dev/sqlite@0.0.2';
import { id } from 'https://r.justjs.dev/id';
import { date, temp, rgbToHex, math, base64 } from 'https://r.justjs.dev/std';

// const { Database } = await require('sqlite');
//
const db = new Database('versions');
//
// db.create('versions', 'id text primary key, version text');
// await cmd.spawn('rustc -V').then((output) => {
// 	db.add('versions', { id: id.secure(), version: output });
// });
//
// console.json(db.get('versions', "where version = '%s'".format(cmd.exec('rustc -V'))), true);
//
// db.rm('versions', '');

console.log(rgbToHex(255, 0, 244));
console.log(date.timeFrom(new Date()));
console.log(date.diff(new Date('2020-10-21'), new Date()));
console.log(date.isWeek(new Date()));
console.log(math.randInt(0, 5));
console.log(math.randFloat(0, 5));
console.log(temp.ctof(15));
console.log(temp.ftoc(15));
console.log('hello world'.reverse());
console.log('{"hello":"world"}'.json()['hello']);
console.log(base64.encode('hello world'));
console.log(base64.decode('d29ybGQgaGVsbG8='));
console.log(base64.test('d29ybGQgaGVsbG8='));
console.log(base64.test('hello world'));
// console.log(id.secure())
