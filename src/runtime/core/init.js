const Just = __bootstrap;
const { ops } = Just.core;

Just.fn = ops;
Just.env_store = {};
Just.fn.async = Just.core.opAsync;
Just.version = ops.runtime_version();
Just.args = ops.env_get('_just_args') ? ops.env_get('_just_args').split(' ') : '';

const init_runtime_env = () => {
	const env_file = Just.args ? Just.args[Just.args.findIndex((i) => i.includes('env='))] : 'env=find';
	const env_object = parseBuffer(ops.env_local(env_file));

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

	delete Object.prototype.__proto__;
	delete Intl.v8BreakIterator;
};
