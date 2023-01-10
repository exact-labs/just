'use strict';

const Just = __bootstrap
const { ops } = Just.core;

Just.core.initializeAsyncOps();

delete Object.prototype.__proto__;
delete Intl.v8BreakIterator;
