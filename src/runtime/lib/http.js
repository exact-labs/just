((globalThis) => {
	const parseBody = (p) => {
		if (Array.isArray(p)) return JSON.stringify(p);
		else if (typeof p == 'string') return p;
		else if (p != null && typeof p == 'object') return JSON.stringify(p);
		else return p;
	};

	globalThis.http = {
		get: (url, headers = { 'User-Agent': 'JustRuntime/' + ops.op_version() }) => Just.core.opAsync('op_get', url, JSON.stringify(headers)),
		post: (url, body = '', headers = { 'User-Agent': 'JustRuntime/' + ops.op_version() }) =>
			Just.core.opAsync('op_post', url, parseBody(body), JSON.stringify(headers)),
	};

	globalThis.server = {
		static: (port = 8080, path) => ops.op_static(port, path),
	};
})(globalThis);
