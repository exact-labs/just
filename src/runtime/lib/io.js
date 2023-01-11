const fs = {
	read: {
		file: (path) => Just.core.opAsync('op_read_file', path),
		dir: (path = './') => Just.core.ops.op_read_dir(path),
	},
	write: {
		file: (path, contents) => Just.core.opAsync('op_write_file', path, contents),
		dir: (path) => Just.core.opAsync('op_make_dir', path),
	},
	remove: {
		file: (path) => Just.core.opAsync('op_remove_file', path),
		dir: (path) => Just.core.opAsync('op_remove_dir', path),
	},
	sha: (path) => Just.core.ops.op_file_sha(path),
};

export { fs };
