((globalThis) => {
	globalThis.fs = {
		read: {
			file: (path) => Just.core.opAsync('op_read_file', path),
			dir: (path = './') => ops.op_read_dir(path),
		},
		write: {
			file: (path, contents) => Just.core.opAsync('op_write_file', path, contents),
			dir: (path) => Just.core.opAsync('op_make_dir', path),
		},
		remove: {
			file: (path) => Just.core.opAsync('op_remove_file', path),
			dir: (path) => Just.core.opAsync('op_remove_dir', path),
		},
		sha: (path) => ops.op_file_sha(path),
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

	globalThis.cmd = {
		exec: (cmd) => ops.op_exec(cmd),
		spawn: async (cmd) => core.opAsync('op_spawn', cmd),
	};
})(globalThis);
