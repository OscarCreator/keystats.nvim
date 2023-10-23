use std::{env, path::Path};

use mlua::prelude::*;
use rusqlite::{params, Connection};

fn add_count(_: &Lua, count: i32) -> LuaResult<()> {
    let conn = Connection::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("keystats.db"))
        .expect("could not connect to database");
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS keystats (
            id INTEGER PRIMARY KEY,
            counter INTEGER DEFAULT 0
        )",
        (),
    )
    .expect("error creating table");

    conn.execute(
        "INSERT OR REPLACE INTO keystats (id, counter)
        VALUES (1, (SELECT IFNULL(SUM(counter), 0) FROM keystats WHERE id == 1) + (?1))",
        params![count],
    )
    .expect("error adding value to database");

    Ok(())
}

fn get_count(_: &Lua, _: ()) -> LuaResult<i32> {
    let conn = Connection::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("keystats.db")).unwrap();
    let res: i32 = conn
        .query_row(
            "SELECT SUM(counter) FROM keystats WHERE id == 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    Ok(res)
}

#[mlua::lua_module]
fn keystats_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("add_count", lua.create_function(add_count)?)?;
    exports.set("get_count", lua.create_function(get_count)?)?;
    Ok(exports)
}
