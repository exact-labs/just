import { cmd } from 'just/sys';
import { Open } from 'just/db:kv';
import { random } from 'just/crypto';

const db = new Open('ver_db');
const id = 'version.rust.' + random.secure();

db.set(id, cmd.exec('rustc -V').trim());
console.log(id, '->', db.get(id));
db.remove(id);
