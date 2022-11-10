((globalThis) => {
	const core = Deno.core;

	const fmt = (...args) => {
		return args
			.map((arg) =>
				JSON.stringify(arg)
					.replace(/\\n/g, '\n')
					.replace(/\\'/g, "'")
					.replace(/\\"/g, '"')
					.replace(/\\&/g, '&')
					.replace(/\\r/g, '\r')
					.replace(/\\t/g, '\t')
					.replace(/\\b/g, '\b')
					.replace(/\\f/g, '\f')
					.slice(1, arg.length + 1)
			)
			.join(' ');
	};

	Object.defineProperty(String.prototype, 'parseBytes', {
		value(decimals = 2) {
			if (!+this) return '0B';
			const c = 0 > decimals ? 0 : decimals,
				d = Math.floor(Math.log(this) / Math.log(1024));
			return `${parseFloat((this / Math.pow(1024, d)).toFixed(c))}${['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'][d]}`;
		},
	});

	globalThis.__dirname = core.opSync('op_dirname');

	globalThis.console = {
		log: (...args) => {
			core.opSync('op_stdout', fmt(...args));
		},
		info: (...args) => {
			core.opSync('op_info', fmt(...args));
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

	globalThis.os = {
		release: () => {
			return core.opSync('op_release');
		},
		platform: () => {
			return core.opSync('op_platform');
		},
		freemem: () => {
			return core.opSync('op_freemem');
		},
		totalmem: () => {
			return core.opSync('op_totalmem');
		},
	};
})(globalThis);
