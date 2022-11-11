((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.internal = () => core;
	globalThis.pkg = (file) => ops.op_packages_dir() + `/${file}/index.js`;
	globalThis.__dirname = ops.op_dirname();
	globalThis.sleep = (ms) => ops.op_sleep(ms);
})(globalThis);
