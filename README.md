> **Vinculum**
> 
> "The processing device at the core of every Borg vessel.
> It interconnects the minds of all the drones.
> It purges individual thoughts and disseminates information relevant to the Collective."
>
> "It brings order to chaos."
> - Seven of Nine and Kathryn Janeway, 2375

# borg-vinculum

[![LICENSE](https://img.shields.io/github/license/trufflepig-forensics/borg-vinculum?color=blue)](LICENSE)
[![dependency status](https://deps.rs/repo/github/trufflepig-forensics/borg-vinculum/status.svg)](https://deps.rs/repo/github/trufflepig-forensics/borg-vinculum)
[![ci status](https://img.shields.io/github/actions/workflow/status/trufflepig-forensics/borg-vinculum/linux.yml?label=CI)](https://github.com/trufflepig-forensics/borg-vinculum/actions/workflows/linux.yml)

## Architecture

The architecture consists of 3 distinct parts:

- borg remote server
- borg-drone clients
- borg-vinculum server

### Borg remote server

The borg remote server is the server on which the backups are saved. It can
provide multiple repositories for multiple clients. It must be reachable by
all other servers and requires `borg` and `sshd` to be installed.

### Borg-vinculum server

The vinculum server is responsible for notifying users about failed backups,
pruning & compacting the backups regularly. It also provides an administration
interface to check the reports from the drones.

### Borg-drone client

Borg-drone is a thin wrapper for `borg` itself to provide functionality to:
- execute pre- and post-hooks
- collect stats information about how much time was spent in each step as
well as stats about the archive provided by borg itself
- notify the vinculum about any occurred error

## Building from source

As there are currently no prebuilt packages, you have to compile all parts of
the projects from source.

To build both (`borg-drone` and `borg-vinculum`) from source, you need to
have `cargo` installed. The recommended way is to use [rustup](https://rustup.rs)
to manage the installation.

The frontend of `borg-vinculum` also requires `npm` or `yarn` to be installed.
Refer to [the official page](https://nodejs.org/en) for installation instructions.

Build `borg-drone` and `borg-vinculum` executables:

```bash
cargo build -r -p borg-drone -p borg-vinculum
```

The resulting binaries can be found in `target/release/`.

Build the frontend for `borg-vinculum`:

```bash
cd borg-vinculum/frontend/
yarn && yarn build
```

or with `npm`:

```bash
cd borg-vinculum/frontend/
npm install && npm run build
```

The resulting files can be found in `dist/`.
