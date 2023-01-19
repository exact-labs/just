const os = {
	release: () => Just.fn.os_release(),
	platform: () => Just.fn.os_platform(),
	machine: () => Just.fn.os_machine(),
	hostname: () => Just.fn.os_hostname(),
	homedir: () => Just.fn.os_homedir(),
	cpus: () => Just.fn.os_cpus(),
	uptime: () => Just.fn.os_uptime(),
	memory: () => Just.fn.os_memory(),
	loadavg: () => Just.fn.os_loadavg(),
	exit: (code) => Just.fn.os_exit(code),
};

const cmd = {
	exec: (cmd) => Just.fn.cmd_exec(cmd),
	spawn: async (cmd) => Just.fn.async('cmd_spawn', cmd),
};

const env = {
	get: (value) => Just.fn.env_get(value),
	set: (key, value) => {
		Just.fn.env_set(key, value);
		Object.defineProperty(Just.env_store, key, { value });
	},
};

export { env, os, cmd };
