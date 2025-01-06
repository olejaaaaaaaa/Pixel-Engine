
use log::info;
use mlua::{Function, Lua, Table};
use std::{fs::File, io::Read};

pub fn debug_exec_script() {

    info!("Запускаю lua скрипт main.lua");
    let mut file = File::open("src/assets/scripts/main.lua").unwrap();
    let mut s = String::new();

    let lua = Lua::new();
    let var = lua.create_table().unwrap();
    var.set("version", "0.0.2");
    file.read_to_string(&mut s);

    lua.globals().set("engine", var);
    lua.load(s).exec().unwrap();
    
    let globals = lua.globals();
    let init: Function = globals.get("init").unwrap();
    init.call::<()>(());
}