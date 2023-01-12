((globalThis) => {
	globalThis.Just = Just;
	globalThis.__dirname = ops.os_dirname();
	globalThis.sleep = (ms) => ops.sleep(ms);
	globalThis.require = (package, version = '') => import('file:///' + ops.get_package(package, version));

	globalThis.core = {
		setup: () => ops.setup(),
		print: (text) => ops.print(text),
		println: (text) => ops.print(`${text}\n`),
		escape: (text) => ops.escape_string(text),
	};
})(globalThis);
