((globalThis) => {
	const core = Deno.core;

	function argsToMessage(...args) {
		return args.map((arg) => JSON.stringify(arg)).join(' ');
	}

	globalThis.console = {
		log: (...args) => {
			core.print(`[out]: ${argsToMessage(...args)}\n`, false);
		},
		error: (...args) => {
			core.print(`[err]: ${argsToMessage(...args)}\n`, true);
		},
		clear: () => {
			core.print('\033[2J\033[1;1H', true);
		},
	};

	globalThis.core = {
		readFile: (path) => {
			return core.opAsync('op_read_file', path);
		},
		writeFile: (path, contents) => {
			return core.opAsync('op_write_file', path, contents);
		},
		removeFile: (path) => {
			return core.opSync('op_remove_file', path);
		},
	};
})(globalThis);
