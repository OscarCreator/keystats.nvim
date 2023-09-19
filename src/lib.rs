use mlua::prelude::*;
use rusqlite::{Connection, params};

fn add_count(_: &Lua, count: i32) -> LuaResult<i32> {
    let conn = Connection::open("keystats.db").unwrap();
    conn.execute("
        CREATE TABLE IF NOT EXISTS keystats (
            id INTEGER PRIMARY KEY,
            counter INTEGER DEFAULT 0
        )", 
        (),
    ).unwrap();

    conn.execute(
        "INSERT OR REPLACE INTO keystats (id, counter)
        VALUES (1, (SELECT IFNULL(SUM(counter), 0) FROM keystats WHERE id == 1) + (?1))",
        params![count]
    ).unwrap();

    let res: i32 = conn.query_row("SELECT SUM(counter) FROM keystats WHERE id == 1", 
        [], 
        |row| row.get(0)).unwrap();

    Ok(res)
}

#[mlua::lua_module]
fn keystats_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("add_count", lua.create_function(add_count)?)?;
    Ok(exports)
}

