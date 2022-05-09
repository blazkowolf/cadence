# Node Nuke

> Delete `node_modules` and any associated lock files with extreme prejudice and swag 😎

## Background

Let me tell you a little story of the corporate landscape.

Picture this, you're a developer working for a large company. You've been assigned a laptop
with an Intel i5 processor. You clone the latest code from your team's repository and shivers
run down your spine when you see it...

A ~1GB `node_modules` folder is staring you in the face, and it looks ornery. You hesitate before
slowly dragging your mouse over the beast, right-clicking it, and selecting delete. Windows 10 wastes
time quantifying the contents, only frightening you more while you anxiously await the folder's
deletion.

Life doesn't have to be this terrifying. If you're proficient with cmd, bash, pwsh, etc. then this may
be of no consequence to you. I'd urge you to stick around anyway.

_Node Nuke_ is a 🅱️lazingly fast, industry disrupting, CLI program enabling you to work smarter by absolutely
annihilating your `node_modules` and simultaneously fist bumping itself.

## Usage

```
nn [OPTIONS] [PATH]
```

### Args

| Arg  | Desc                                                                                                       |
| ---- | ---------------------------------------------------------------------------------------------------------- |
| PATH | Path to a directory containing a `node_modules` folder. Defaults to the current directory if not provided. |

### Options

| Option              | Desc                             |
| ------------------- | -------------------------------- |
| `-D, --remove-lock` | Remove all associated lock files |
| `-h, --help`        | Print help information           |
| `-V, --version`     | Print version information        |

## FAQ

> ❓ What about other solutions like `rm -rf node_modules && rm package-lock.json`?

No

> ❓ But `rd /s/q .\node_modules && del pack...`

_No_
