'use strict';

const { ops } = __bootstrap.core;

delete Object.prototype.__proto__;
delete Intl.v8BreakIterator;

Deno.core.initializeAsyncOps();

((globalThis) => {
	globalThis.runtime = {
		version: () => ops.runtime_version(),
		internal: __bootstrap,
	};
})(globalThis);
