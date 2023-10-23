# keystats.nvim
Do you know how many key presses you do every day?

## Dependencies

- [cargo](https://www.rust-lang.org/tools/install)

## Installation

```lua
use {
    'OscarCreator/keystats.nvim',
    run = 'make',
}
```

## Usage

**keystats.nvim** uses `vim.on_key()` to count and then save you key presses to
a database.
