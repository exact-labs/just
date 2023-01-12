export class Open {
	constructor(path) {
		return {
			get: (key) => Just.fn.kv_get(path, key),
			set: (key, value) => Just.fn.kv_set(path, key, value),
			remove: (key) => Just.fn.kv_remove(path, key),
			range: (start, end) => Just.fn.kv_range(path, start, end),
		};
	}
}
