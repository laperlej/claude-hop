# claude-hop

Fuzzy picker for jumping between parallel Claude Code instances.

Run multiple Claude Code sessions in tmux/zellij panes and instantly see which ones need attention. One keystroke to jump to an idle instance.

## Features

- **Picker** — fuzzy search across all running Claude instances, Enter to jump to that pane
- **Jump** — instantly switch to the most recently idle instance (no UI)
- **Alerts** — desktop notification when a non-focused instance goes idle (via Claude hooks)

## Install

```sh
cargo install claude-hop
```

## Usage

### Picker

```sh
claude-hop pick
```

Opens an interactive fuzzy picker showing all Claude instances with status indicators:

- `●` idle (waiting for input)
- `⚡` busy (working)
- `⏸` permission (waiting for approval)

Type to fuzzy filter. Enter to jump. Esc to close.

### Jump to latest idle

```sh
claude-hop jump
```

No UI — immediately switches to the most recently idle instance that isn't in your current pane.

### Recommended keybindings

**tmux** (add to `~/.tmux.conf`):

```tmux
bind-key F display-popup -E "claude-hop pick"
bind-key J run-shell "claude-hop jump"
```

**zellij** (add to `config.kdl`):

```kdl
bind "Alt f" {
    Run "claude-hop" "pick" {
        in_place true
    }
}
```

## Alerts (optional)

Configure Claude Code hooks for instant notifications when an instance goes idle.

```sh
claude-hop setup
```

This prints the hook configuration to add to `~/.claude/settings.json`.

## How it works

1. Reads Claude session files from `~/.claude/sessions/*.json`
2. Correlates Claude PIDs to multiplexer panes via process tree walking
3. Renders a ratatui fuzzy picker (or jumps directly with `jump`)
4. Switches your multiplexer to the selected pane

Works with tmux and zellij. Falls back to displaying PID + working directory if no multiplexer is detected.

## License

MIT
