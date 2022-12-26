((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.require = (package, version = '') => import('file:///' + ops.op_get_package(package, version));
	globalThis.__dirname = ops.op_dirname();
	globalThis.sleep = (ms) => ops.op_sleep(ms);

	globalThis.core = {
		print: (text) => ops.op_print(text),
		encode: (text) => ops.op_encode(text),
		encode_fast: (text) => ops.op_encode_fast(text),
		escape: (text) => ops.op_escape(text),
	};
})(globalThis);
