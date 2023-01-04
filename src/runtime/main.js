'use strict';

delete Object.prototype.__proto__;
delete Intl.v8BreakIterator;

Deno.core.initializeAsyncOps();

((globalThis) => {
	globalThis.runtime = {
		version: () => Deno.core.ops.op_version(),
		internal: Deno,
	};
})(globalThis);
