# knife

> A terminal application to find and delete your old, deserted GitHub repositories.

![image](https://github.com/strbrgr/knife/blob/main/assets/knife_jochen_stierberger.jpg)

## Features

- Authentication using a GitHub token
- Lists your personal repositories in a scrollable TUI
- Mark repositories for deletion using keyboard navigation
- Easily clean up inactive or forgotten repos
- Beautiful terminal UI powered by [ratatui](https://github.com/ratatui-org/ratatui)

## Demo

Coming soon...

## Installation

Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs).

```bash
git clone https://github.com/strbrgr/knife.git
cd knife
cargo run
```

## Q&A

### I deleted repos by accident, what now?

As long as you still have the `.git` folder in your project, you will be able to create another repository and link it your new upstream.
