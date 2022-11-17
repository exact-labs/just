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
				get: (table, query) =>
					JSON.parse(
						`[${ops
							.op_db_query(dbPath, table, query)
							.slice(2, -2)
							.split('}", "{')
							.join('},{')
							.replaceAll('\\', '')
							.replace(/"(-|)([0-9]+(?:\.[0-9]+)?)"/g, '$1$2')}]`
					),
			};

			return functions;
		}
	};
})(globalThis);
