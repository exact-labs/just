import { os } from 'just/sys';

const fn_list = [os.release, os.platform, os.machine, os.hostname, os.homedir, os.cpus, os.uptime, os.memory, os.loadavg];
const get_name = (fn_name) => `os.${fn_name.toString().split('.').pop().split('_')[1].slice(0, -2)}: `;

fn_list.map((fn_name) => {
	console.fmt.hex.print('#D9A7F7', get_name(fn_name));
	console.log(fn_name());
});

console.fmt.hex.print('#D9A7F7', 'Just.mem: ');
console.log(Just.mem());
await sleep(2000);
console.log(Just.mem());
