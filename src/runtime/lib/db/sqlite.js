const formatJson = (string) =>
	JSON.parse(
		`[${string
			.slice(2, -2)
			.split('}", "{')
			.join('},{')
			.replaceAll('\\', '')
			.replace(/"(-|)([0-9]+(?:\.[0-9]+)?)"/g, '$1$2')}]`
	);

export class Database {
	constructor(filename) {
		const dbPath = filename ? `${filename}.db` : ':memory:';
		ops.sqlite_init(dbPath);

		var functions = {
			create: (table, keys) => ops.sqlite_create(dbPath, table, keys),
			query: (table, query) => formatJson(ops.sqlite_query(dbPath, table, query)),
			insert: (table, query) => ops.sqlite_insert(dbPath, table, Object.keys(query).join(', '), `'${Object.values(query).join("', '")}'`),
			delete: (table, query) => ops.sqlite_delete(dbPath, table, query),
			exec: (query) => ops.sqlite_exec(dbPath, query),
		};
		return functions;
	}
}
