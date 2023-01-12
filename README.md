# Just

![crates.io](https://img.shields.io/crates/v/justjs.svg?style=flat-square)

<img align="right" src="https://cdn.justjs.dev/assets/svg/logo.svg" height="150px" alt="just circle logo">

Just is a _simple_, and _modern_ runtime for **JavaScript** that uses V8 and is built in Rust.

### Features

- Multiple ways to import files.
- External language support [like go](https://github.com/exact-rs/just/tree/master/src/go).
- Built code formatter and task runner.
- Builds a self-contained executable.
- Useful [built-in](https://justjs.dev/docs/tools) utilities.
- Includes a set of standard modules for [Just](https://justjs.dev/r/std).

### Install

Shell (Mac, Linux):

```sh
curl -fsSL https://justjs.dev/install.sh | sh
```

PowerShell (Windows):

```powershell
irm https://justjs.dev/install.ps1 | iex
```

[Homebrew](https://formulae.brew.sh/formula/justjs) (Mac):

```sh
brew install justjs
```

[Chocolatey](https://chocolatey.org/packages/justjs) (Windows):

```powershell
choco install justjs
```

[Scoop](https://scoop.sh/) (Windows):

```powershell
scoop install justjs
```

Build and install from source using [Cargo](https://crates.io/crates/justjs):

```sh
cargo install justjs --locked
```

### Getting Started

Try running a simple program:

```sh
just run https://r.justjs.dev/std/examples/welcome.js
```

Or a more complex one:

```js
import { cmd } from 'just/sys';
import { random } from 'just/crypto';
import { Database } from 'https://r.justjs.dev/sqlite';

const db = new Database('db_name');

db.create('versions', 'id text primary key, version text');
await cmd.spawn('just -v').then((output) => {
	db.add('versions', { id: random.secure(), version: output });
});

console.json(db.get('versions', "where version = '%s'".format(cmd.exec('just -v'))), true);
```

Just package registry can be located [here](https://justjs.dev/r/) ([api](https://r.justjs.dev)).

You can find a deeper introduction, examples, and environment setup guides in
the [docs](https://justjs.dev/docs).

The complete API reference is available at the runtime
[documentation](https://justjs.dev/docs/api).
