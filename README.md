# claude-hop

Fuzzy picker for jumping between parallel Claude Code instances in tmux.

See which instances need attention, jump to them with one keystroke.

## Usage

### Picker (`prefix + F`)

Opens a popup with all running Claude instances and their status.

- Up/Down to navigate
- Type to fuzzy filter
- Enter to jump to selected instance
- Esc to close

Status indicators:

- `[idle]` waiting for input
- `[busy]` working
- `[ask]` waiting for user response (questionnaire)
- `[wait]` waiting

### Jump (`prefix + J`)

No UI — immediately switches to the most recently idle instance that isn't in your current pane.

### Status bar

Shows when an instance is ready for attention:

```
⏎ my-project          (one instance ready)
⏎ 3 ready             (multiple instances ready)
                       (nothing to do — hidden)
```

## Installation

```bash
cargo install claude-hop
```

Or build from source:

```bash
git clone https://github.com/laperlej/claude-hop.git
cd claude-hop
cargo build --release
```

## Configuration

### tmux keybindings

Add to `~/.tmux.conf`:

```tmux
# picker popup
bind F display-popup -E "claude-hop pick"

# jump to latest idle instance
bind J run-shell "claude-hop jump"
```

### tmux status bar

Add to `~/.tmux.conf`:

```tmux
set -g status-left '#(claude-hop status)'
set -g status-left-length 40
set -g status-interval 2
```

### Claude Code hooks (optional)

For desktop notifications when a non-focused instance goes idle:

```bash
claude-hop setup
```

This prints the hook configuration to add to `~/.claude/settings.json`.

## How it works

1. Reads Claude session files from `~/.claude/sessions/*.json`
2. Correlates Claude PIDs to tmux panes via process tree walking
3. Renders a [ratatui](https://github.com/ratatui/ratatui) fuzzy picker with [nucleo](https://github.com/helix-editor/nucleo) matching
4. Switches your tmux client to the selected pane

## Contributing

Contributions are welcome. Please open an issue or a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
