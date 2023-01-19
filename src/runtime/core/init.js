const Just = __bootstrap;
const { ops } = Just.core;

const build = {
	target: 'unknown',
	arch: 'unknown',
	os: 'unknown',
	vendor: 'unknown',
};

const set_build_info = (target) => {
	const [arch, vendor, os] = target.split('-', 3);

	build.target = target;
	build.arch = arch;
	build.vendor = vendor;
	build.os = os;

	Object.freeze(build);
};

Just.fn = ops;
Just.build = build;
Just.env_store = {};
Just.fn.async = Just.core.opAsync;
Just.options = JSON.parse(ops.options());
Just.args = ops.env_get('_just_args') ? ops.env_get('_just_args').split(' ') : '';
Just.mem = () => ops.runtime_memory();

const init_runtime_env = () => {
	const env_index = Just.args ? Just.args.findIndex((i) => i.includes('env=')) : -1;
	const env_path = env_index != -1 ? Just.args[env_index] : 'env=find';
	const env_object = parseBuffer(ops.env_local(env_path));

	Object.keys(env_object || {}).map((key) => {
		ops.env_set(key, env_object[key]);
		Object.defineProperty(Just.env_store, key, {
			value: env_object[key],
			enumerable: true,
			writable: true,
		});
	});
};

const init_runtime_global = () => {
	Just.core.initializeAsyncOps();
	init_runtime_env();
	set_build_info(Just.options.target);

	delete Object.prototype.__proto__;
	delete Intl.v8BreakIterator;
};
