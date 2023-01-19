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
	exec: (name, args = null, path = Just.fn.os_dirname()) =>
		args == null ? Just.fn.cmd_exec(name.split(' ')[0], name.split(' ').slice(1), path) : Just.fn.cmd_exec(name, args, path),
	spawn: async (name, args = null, path = Just.fn.os_dirname()) =>
		args == null ? Just.fn.async('cmd_spawn', name.split(' ')[0], name.split(' ').slice(1), path) : Just.fn.async('cmd_spawn', name, args, path),
};

const env = {
	get: (value) => Just.fn.env_get(value),
	set: (key, value) => {
		Just.fn.env_set(key, value);
		Object.defineProperty(Just.env_store, key, { value });
	},
};

export { env, os, cmd };
