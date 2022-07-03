use std::collections::BTreeMap;
use mlua::prelude::*;
use mlua::{Lua, Result, Table, Value, ToLua};
use serde_dhall::{SimpleValue, NumKind, from_str};

fn list_to_table<'lua>(lua: &'lua Lua, dhall_list: &Vec<SimpleValue>) -> Table<'lua> {
    dhall_list.iter().enumerate().fold(
        lua.create_table(),
        |table, (k, v)| table.and_then(
            |t| dhall_value_to_lua(lua, v).and_then(
                |lua_value| t.set(k+1, lua_value).map(|_| t)))).unwrap()
}

fn record_to_table<'lua>(lua: &'lua Lua, 
                         dhall_record: &BTreeMap<String, SimpleValue>) -> Table<'lua> {
    dhall_record.iter().fold(
        lua.create_table(),
        |table, (k, v)| lua.create_string(k).and_then(
            |key_string| table
                .and_then(|t|dhall_value_to_lua(lua, v) 
                    .and_then(|lua_value| t
                           .set(key_string, lua_value)
                           .map(|_| t))))).unwrap()
}


fn dhall_value_to_lua<'lua>(lua: &'lua Lua, dhall_value: &SimpleValue) -> Result<Value<'lua>> {
    return match dhall_value {
        SimpleValue::Num(n) => match n {
            NumKind::Bool(b) => (*b).to_lua(lua),
            NumKind::Natural(i) => (*i).to_lua(lua),
            NumKind::Integer(i) => (*i).to_lua(lua),
            NumKind::Double(d) => f64::from(*d).to_lua(lua),
        },
        SimpleValue::Text(t) => Ok(Value::String(lua.create_string(t).unwrap())) ,
        SimpleValue::Optional(o) => match o {
            None => Ok(Value::Nil),
            Some(v) => dhall_value_to_lua(lua,v)

        }
        SimpleValue::List(l) => Ok(Value::Table(list_to_table(lua, l))),
        SimpleValue::Record(r) => Ok(Value::Table(record_to_table(lua, r))),
        SimpleValue::Union(name, value) => match value {
            None => Ok(Value::String(lua.create_string(name).unwrap())),
            Some(x) => dhall_value_to_lua(lua, x.as_ref())
        }
    }
}

fn load_string(lua: &Lua, dhall_code: String) -> Result<Value> {
    match from_str(&dhall_code).parse() {
        Ok(data) => dhall_value_to_lua(lua, &data),
        Err(e) => {
            Err(LuaError::RuntimeError(format!("Error reading dhall: \n{e}\n")))
        }
    }
}


#[mlua::lua_module]
fn dhall_lua(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    exports.set("load_string", lua.create_function(load_string)?)?;
    Ok(exports)
}
