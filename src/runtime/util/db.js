((globalThis) => {
	const { core } = Deno;
	const { ops } = core;

	globalThis.Database = class Database {
		constructor(filename) {
			const dbPath = filename ? `${filename}.db` : ':memory:';
			ops.op_db_init(dbPath);

			var functions = {
				create: (table, keys) => ops.op_db_create(dbPath, table, keys),
				add: (table, query) => ops.op_db_insert(dbPath, table, Object.keys(query).join(', '), `'${Object.values(query).join("', '")}'`),
				get: (table, query) => ops.op_db_query(dbPath, table, query),
			};

			return functions;
		}
	};
})(globalThis);
