# knife

<!--toc:start-->

- [knife](#knife)
  - [Features](#features)
  - [Installation](#installation)
  - [Shoutouts](#shoutouts)
  - [Q&A](#qa)
  <!--toc:end-->

> A terminal application to find and delete your old, deserted GitHub repositories.

![image](https://github.com/strbrgr/knife/blob/main/assets/knife_jochen_stierberger.jpg)

## Features

- Authentication using a GitHub token
- Lists your personal repositories in a scrollable TUI
- Mark repositories for deletion using keyboard navigation
- Easily clean up inactive or forgotten repos
- Beautiful terminal UI powered by [ratatui](https://github.com/ratatui-org/ratatui)

## Installation

Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs).

```bash
git clone https://github.com/strbrgr/knife.git
cd knife
cargo run
```

## Shoutouts

- [Orhun Parmaksız](https://github.com/orhun) for his work on [ratatui](https://github.com/ratatui-org/ratatui)
- [Josh McKinney](https://github.com/joshka) for writing a lot of high quality code to learn from

## Q&A

### I deleted repos by accident, what now?

As long as you still have the `.git` folder in your project, you will be able to create another repository and link it your new upstream.
