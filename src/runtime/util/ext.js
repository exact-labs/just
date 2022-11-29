((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.ext = {
		raw: (raw) => core.run_ext_func(raw),
		file: {
			read: (path) => ops.run_ext_func('get_file:' + path),
		},
	};
})(globalThis);
