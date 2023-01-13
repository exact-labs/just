import { fs } from 'just/io';
import { os, cmd } from 'just/sys';
import { uuid, random } from 'just/crypto';
import { base64, hex } from 'just/crypto:enc';
import { Open } from 'just/db:kv';
import { run } from './net.ts';

console.fmt.hex.print('#D9A7F7', 'just_args: ');
console.log(Just.args);

await fs.read.file('main.go').then((code) => {
	console.fmt.hex.print('#D9A7F7', 'sha_of_file: ');
	console.log(fs.sha('main.go'));

	console.fmt.hex.print('#D9A7F7', 'eval_go_code: ');
	go.eval(code);
});

console.fmt.hex.print('#D9A7F7', 'base64_import: ');
const base64_import = await import(process.env.BASE64);
base64_import.helloWorld();

console.fmt.hex.print('#D9A7F7', 'string_import: ');
const string_import = await import(process.env.STRING);
string_import.helloWorld();

await import(`data:text/javascript,${encodeURIComponent(process.env.DATA)}`).then((res) => {
	console.fmt.hex.print('#D9A7F7', 'data_import: ');
	console.log(res.string);
});

console.fmt.hex.print('#D9A7F7', 'uuid_v4: ');
console.log(uuid());

console.fmt.hex.print('#D9A7F7', 'secure_id: ');
console.log(random.secure());

console.fmt.hex.print('#D9A7F7', 'basic_id: ');
console.log(random.basic());

console.fmt.hex.print('#D9A7F7', 'convert_to_bytes: ');
console.log('hello world'.to_bytes());

console.fmt.hex.print('#D9A7F7', 'convert_from_bytes: ');
console.log([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100].from_bytes());

console.fmt.hex.print('#D9A7F7', 'format_string: ');
console.log('hello %s'.format('world'));

console.fmt.hex.print('#D9A7F7', 'parse_memory_string: ');
console.log('2147483648'.parse_memory());

console.fmt.hex.print('#D9A7F7', 'encode_hex: ');
console.log(hex.encode('hello world'));

console.fmt.hex.print('#D9A7F7', 'decode_hex: ');
console.log(hex.decode('68656c6c6f20776f726c64'));

console.fmt.hex.print('#D9A7F7', 'encode_base64: ');
console.log(base64.encode('hello world'));

console.fmt.hex.print('#D9A7F7', 'decode_base64: ');
console.log(base64.decode('aGVsbG8gd29ybGQ='));

console.fmt.hex.print('#D9A7F7', 'os_platform: ');
console.log(os.platform);

const db = new Open('ver_db');
console.fmt.hex.print('#D9A7F7', 'kv_set_str: ');
db.set('str', 'hello world');
console.log(db.get('str'));
db.remove('str');

console.fmt.hex.print('#D9A7F7', 'kv_set_json: ');
db.set('json', { hello: 'world' });
console.log(db.get('json'));
db.remove('json');

console.fmt.hex.print('#D9A7F7', 'kv_set_int: ');
db.set('number', 500);
console.log(db.get('number'));
db.remove('number');

console.fmt.hex.print('#D9A7F7', 'command_exec: ');
console.log(cmd.exec('rustc --version'));

console.fmt.hex.print('#D9A7F7', 'import_ts_net: ');
run();
