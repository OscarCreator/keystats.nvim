local M = {}

local db = require("keystats_nvim")
local config = require("keystats.config")
local uv = vim.loop

local count = 0

local async = uv.new_async(function(args)
    db.add_count(args)
end)

vim.on_key(function(char)
    count = count + 1
    -- local x = vim.fn.reltime()
    if count >= config.values.save_count then
        async:send(count)
        count = 0
    end
    -- local y = vim.fn.reltime(x)
    -- vim.print(string.format("elapsed time: %.6f\n", vim.fn.reltimestr(y)))
end)

function M.get_count()
    return db.get_count() + count
end

function M.setup(user_config)
    config.set_defaults(user_config)
end

return M
