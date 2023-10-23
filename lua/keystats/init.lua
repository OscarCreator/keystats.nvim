local M = {}

local db = require("keystats_nvim")
local uv = vim.loop

local async

async = uv.new_async(function()
    db.add_count(1)
end)

vim.on_key(function()
    async:send()
end)

function M.get_count()
    return db.get_count()
end

return M
