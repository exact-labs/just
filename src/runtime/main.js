'use strict';

const { ops } = __bootstrap.core;

__bootstrap.core.initializeAsyncOps();

delete Object.prototype.__proto__;
delete Intl.v8BreakIterator;
