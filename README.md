# Hevy Bridge

A Rust-based CLI client for [Hevy's API](https://api.hevyapp.com/docs/#/). Designed as a straightforward tool for humans and AI agents alike.

## Prerequisites

- **Rust** (1.85+ for edition 2024)
- **OpenSSL dev headers** — `openssl-devel` (Fedora/RHEL) or `libssl-dev` (Debian/Ubuntu)
- **Hevy Pro account** — API keys are available at <https://hevy.com/settings?developer>

## Installation

```bash
git clone https://github.com/Camelron/hevy-bridge.git
cd hevy-bridge
cargo build --release
# Binary is at ./target/release/hevy-bridge
```

## Configuration

Provide your API key via one of these methods (checked in this order):

1. **Flag** — `--api-key <KEY>` on any command
2. **Environment variable** — `export HEVY_API_KEY=<KEY>`
3. **Persisted config** — run once:
   ```bash
   hevy-bridge config set-key <YOUR_API_KEY>
   ```
   Saves to `~/.config/hevy-bridge/config.json`.

## Usage

All data commands output JSON to stdout for easy piping to `jq` or scripts.

```bash
# Save your API key
hevy-bridge config set-key YOUR_API_KEY

# User info
hevy-bridge user info

# Workouts
hevy-bridge workouts list --page 1 --page-size 5
hevy-bridge workouts get <WORKOUT_ID>
hevy-bridge workouts count
hevy-bridge workouts events --since 2024-01-01T00:00:00Z
hevy-bridge workouts create --json '{"workout":{...}}'
hevy-bridge workouts update <WORKOUT_ID> --json '{"workout":{...}}'

# Routines
hevy-bridge routines list
hevy-bridge routines get <ROUTINE_ID>
hevy-bridge routines create --json '{"routine":{...}}'
hevy-bridge routines update <ROUTINE_ID> --json '{"routine":{...}}'

# Exercise templates
hevy-bridge exercises list --page-size 100
hevy-bridge exercises get <TEMPLATE_ID>
hevy-bridge exercises create --json '{"exercise":{...}}'

# Routine folders
hevy-bridge folders list
hevy-bridge folders get <FOLDER_ID>
hevy-bridge folders create --json '{"routine_folder":{"title":"My Folder"}}'

# Exercise history
hevy-bridge history get <TEMPLATE_ID>
hevy-bridge history get <TEMPLATE_ID> --start 2024-01-01T00:00:00Z --end 2024-12-31T23:59:59Z
```

## Detailed Help

Every command and subcommand includes full schema documentation:

```bash
hevy-bridge --help
hevy-bridge workouts --help
hevy-bridge workouts create --help
```

## For AI Agents

- Use `hevy-bridge exercises list --page-size 100` to discover `exercise_template_id` values needed when creating workouts or routines.
- POST/PUT bodies are passed as raw JSON via the `--json` flag.
- Pagination: use `--page` and `--page-size` on list commands; check the returned `page_count` to know when to stop.
- Dates use ISO 8601 format: `2024-01-15T00:00:00Z`.
- All output is JSON printed to stdout; status messages go to stderr. 