const fs = {
	read: {
		file: (path) => Just.fn.async('op_read_file', path),
		dir: (path = './') => Just.fn.op_read_dir(path),
	},
	write: {
		file: (path, contents) => Just.fn.async('op_write_file', path, contents),
		dir: (path) => Just.fn.async('op_make_dir', path),
	},
	remove: {
		file: (path) => Just.fn.async('op_remove_file', path),
		dir: (path) => Just.fn.async('op_remove_dir', path),
	},
	sha: (path) => Just.fn.op_file_sha(path),
};

export { fs };
