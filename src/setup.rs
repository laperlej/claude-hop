use anyhow::Result;

pub fn run() -> Result<()> {
    println!("Claude Hop Setup");
    println!("================");
    println!();
    println!("1. Add hooks to ~/.claude/settings.json:");
    println!();
    println!(r#"  "hooks": {{"#);
    println!(r#"    "Notification": ["#);
    println!(r#"      {{"#);
    println!(r#"        "matcher": "idle_prompt","#);
    println!(r#"        "hooks": [{{"#);
    println!(r#"          "type": "command","#);
    println!(r#"          "command": "claude-hop event idle""#);
    println!(r#"        }}]"#);
    println!(r#"      }},"#);
    println!(r#"      {{"#);
    println!(r#"        "matcher": "permission_prompt","#);
    println!(r#"        "hooks": [{{"#);
    println!(r#"          "type": "command","#);
    println!(r#"          "command": "claude-hop event asking""#);
    println!(r#"        }}]"#);
    println!(r#"      }}"#);
    println!(r#"    ]"#);
    println!(r#"  }}"#);
    println!();
    println!("2. Add tmux keybindings to ~/.tmux.conf:");
    println!();
    println!(r#"  bind-key F display-popup -E "claude-hop pick""#);
    println!(r#"  bind-key J run-shell "claude-hop jump""#);
    println!();
    Ok(())
}
