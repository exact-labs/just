((globalThis) => {
	globalThis.go = {
		eval: (code) => ops.print(ops.external_function('eval_go', encodeURIComponent(code))),
		file: {
			read: (path) => ops.external_function('get_file', `--path "${path}"`),
		},
	};
})(globalThis);
