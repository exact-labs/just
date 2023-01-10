((globalThis) => {
	globalThis.require = (package, version = '') => import('file:///' + ops.get_package(package, version));
	globalThis.__dirname = ops.op_dirname();
	globalThis.sleep = (ms) => ops.sleep(ms);

	globalThis.core = {
		print: (text) => ops.print(text),
		println: (text) => ops.print(`${text}\n`),
		encode: (text) => ops.op_encode(text),
		encode_fast: (text) => ops.op_encode_fast(text),
		escape: (text) => ops.escape_string(text),
	};
})(globalThis);
