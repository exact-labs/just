const parseBody = (p) => {
	if (Array.isArray(p)) return JSON.stringify(p);
	else if (typeof p == 'string') return p;
	else if (p != null && typeof p == 'object') return JSON.stringify(p);
	else return p;
};

const http = {
	get: (url, headers = { 'User-Agent': 'JustRuntime/' + ops.runtime_version() }) => Just.core.opAsync('op_get', url, JSON.stringify(headers)),
	post: (url, body = '', headers = { 'User-Agent': 'JustRuntime/' + ops.runtime_version() }) =>
		Just.core.opAsync('op_post', url, parseBody(body), JSON.stringify(headers)),
};

const server = {
	static: (port = 8080, path) => ops.op_static(port, path),
};

export { server, http };
