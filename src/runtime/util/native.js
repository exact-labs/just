((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.fs = {
		readFile: (path) => core.opAsync('op_read_file', path),
		readDir: (path = './') => ops.op_read_dir(path),
		writeFile: (path, contents) => core.opAsync('op_write_file', path, contents),
		removeFile: (path) => ops.op_remove_file(path),
	};

	globalThis.os = {
		release: () => ops.op_release(),
		platform: () => ops.op_platform(),
		machine: () => ops.op_machine(),
		hostname: () => ops.op_hostname().slice(1, -1),
		homedir: () => ops.op_homedir(),
		cpus: () => ops.op_cpus(),
		uptime: () => ops.op_uptime(),
		freemem: () => ops.op_freemem(),
		totalmem: () => ops.op_totalmem(),
		loadavg: () => ops.op_loadavg(),
		exit: (code) => ops.op_exit(code),
	};

	globalThis.process = {
		env: {
			get: (value) => ops.op_env_get(value),
			set: (key, value) => ops.op_env_set(key, value),
		},
		cwd: () => ops.op_dirname(),
	};
})(globalThis);
