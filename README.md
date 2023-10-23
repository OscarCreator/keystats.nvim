# keystats.nvim
Do you know how many key presses you do every day?

## Dependencies

- [cargo](https://www.rust-lang.org/tools/install)

## Installation

```lua
use {
    'OscarCreator/keystats.nvim',
    run = 'make',
    config = function()
        require("keystats").setup()
    end
}
```

## Usage

**keystats.nvim** uses `vim.on_key()` to count and then save you key presses to
a database.

## Configuration

```lua
{
    -- how often (in strokes) to save to database
    save_count = 10,
}
```
