((globalThis) => {
	const core = Deno.core;

	const fmt = (...args) => {
		return args
			.map((arg) => JSON.stringify(arg))
			.join(' ')
			.slice(1, -1);
	};

	globalThis.console = {
		log: (...args) => {
			core.opSync('op_stdout', fmt(...args));
		},
		error: (...args) => {
			core.opSync('op_stderr', fmt(...args));
		},
		clear: () => {
			core.opSync('op_stdout', '\033[2J\033[1;1H');
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
		sleep: (ms) => {
			core.opSync('op_sleep', ms);
		},
	};
})(globalThis);
