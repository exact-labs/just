const os = {
	release: Just.core.ops.op_release(),
	platform: Just.core.ops.op_platform(),
	machine: Just.core.ops.op_machine(),
	hostname: Just.core.ops.op_hostname().slice(1, -1),
	homedir: Just.core.ops.op_homedir(),
	cpus: Just.core.ops.op_cpus(),
	uptime: Just.core.ops.op_uptime(),
	freemem: Just.core.ops.op_freemem(),
	totalmem: Just.core.ops.op_totalmem(),
	loadavg: Just.core.ops.op_loadavg(),
	exit: (code) => Just.core.ops.op_exit(code),
};

const cmd = {
	exec: (cmd) => Just.core.ops.op_exec(cmd),
	spawn: async (cmd) => Just.core.opAsync('op_spawn', cmd),
};

const env = {
	get: (value) => Just.core.ops.op_env_get(value),
	set: (key, value) => {
		Just.core.ops.op_env_set(key, value);
		Object.defineProperty(Just.env_store, key, { value });
	},
};

export { env, os, cmd };
