# Claude Fleet Monitor

Single Rust binary for tracking parallel Claude Code instances. Three features:

1. **Picker** — fuzzy popup showing all instances + status, Enter to jump
2. **Alert** — desktop notification when a non-focused instance goes idle
3. **Jump to latest** — instantly switch to the most recently finished idle instance

## UX

### Picker (`claude-fleet pick`)

Launched via keybinding (e.g. tmux `bind-key F display-popup -E "claude-fleet pick"`).
Same interaction model as zellij-sessionizer: flat list, fuzzy search, Enter to jump.

```
  ● auth-refactor       ~/Projects/api
  ● db-migration        ~/Projects/db
  ⚡ feature-x           ~/Projects/web
  ⏸ test-runner         ~/Projects/core

  > search...

  ● idle  ⚡ busy  ⏸ permission
```

- j/k or arrows to navigate
- Type to fuzzy filter (nucleo)
- Enter → switch to that pane
- Esc → close
- Idle instances sorted to top

### Alert

Claude hook fires on `idle_prompt` / `permission_prompt` → `claude-fleet event` → desktop notification if that instance is not in the currently focused pane.

- macOS: `osascript` / terminal-notifier
- Linux: `notify-send`

### Jump to latest (`claude-fleet jump`)

No UI. Finds the most recently idle instance that isn't in the current pane, switches to it.

Keybinding: e.g. tmux `bind-key J run-shell "claude-fleet jump"`

## Data sources

### Primary: `~/.claude/sessions/*.json` (polling)

```json
{
  "pid": 35667,
  "sessionId": "001e64a0-...",
  "cwd": "/Users/jonathan/Projects/ai-engineer",
  "status": "busy",
  "startedAt": 1779583418000,
  "kind": "interactive"
}
```

Works with zero setup. Poll on picker open — no daemon needed.

### Optional: Claude hooks (instant alerts)

Hook config for `~/.claude/settings.json`:
```json
{
  "hooks": {
    "Notification": [{
      "matcher": "idle_prompt",
      "hooks": [{ "type": "command", "command": "claude-fleet event idle" }]
    }, {
      "matcher": "permission_prompt",
      "hooks": [{ "type": "command", "command": "claude-fleet event permission" }]
    }],
    "SessionStart": [{
      "hooks": [{ "type": "command", "command": "claude-fleet event start" }]
    }],
    "SessionEnd": [{
      "hooks": [{ "type": "command", "command": "claude-fleet event end" }]
    }]
  }
}
```

`claude-fleet event` reads hook JSON from stdin, writes to `~/.cache/claude-fleet/events.json`, fires desktop notification if instance not focused.

Without hooks: picker still works (reads session files), but no alerts and `jump` relies on polling.

## PID → pane mapping

On `pick` / `jump`, walk process tree from Claude PID up to find the multiplexer pane:
- **tmux:** `tmux list-panes -a -F '#{pane_pid} ...'`, match ancestor PID
- **zellij:** `zellij action list-clients` or process tree
- **none:** show PID + cwd, no jump

Detect multiplexer via `$TMUX` / `$ZELLIJ` env vars.

## Tech stack

- Rust
- ratatui + crossterm (picker UI)
- nucleo (fuzzy matching)
- serde_json (session file + hook parsing)
- sysinfo or ps (process tree)

## CLI

```
claude-fleet pick       # open picker popup
claude-fleet jump       # jump to latest idle instance
claude-fleet event <t>  # hook target (stdin JSON → state + notify)
claude-fleet setup      # print hook config + keybinding instructions
```

## Distribution

- `cargo install claude-fleet`
- Homebrew formula
- GitHub releases (cross-compiled binaries)

## Open questions

- Naming: `claude-fleet`, `cfleet`, `fleetview`?
- Support Codex / other agents, or Claude-only?
