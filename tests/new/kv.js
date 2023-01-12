import { cmd } from 'just/sys';
import { Open } from 'just/db:kv';

const db = new Open('ver_db');

db.set('version.rust', cmd.exec('rustc -V'));
console.log(db.get('version.rust'));
db.remove('version.rust');
