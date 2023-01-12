const os = {
	release: Just.fn.op_release(),
	platform: Just.fn.op_platform(),
	machine: Just.fn.op_machine(),
	hostname: Just.fn.op_hostname().slice(1, -1),
	homedir: Just.fn.op_homedir(),
	cpus: Just.fn.op_cpus(),
	uptime: Just.fn.op_uptime(),
	freemem: Just.fn.op_freemem(),
	totalmem: Just.fn.op_totalmem(),
	loadavg: Just.fn.op_loadavg(),
	exit: (code) => Just.fn.op_exit(code),
};

const cmd = {
	exec: (cmd) => Just.fn.op_exec(cmd),
	spawn: async (cmd) => Just.fn.async('op_spawn', cmd),
};

const env = {
	get: (value) => Just.fn.op_env_get(value),
	set: (key, value) => {
		Just.fn.op_env_set(key, value);
		Object.defineProperty(Just.env_store, key, { value });
	},
};

export { env, os, cmd };
