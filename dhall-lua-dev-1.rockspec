package = "dhall-lua"
version = "dev-1"

source = {
   url = "git+https://github.com/abhashs/dhall-lua.git",
   tag = "0.1"
}

description = {
   summary = "Lua bindings for dhall configuration language",
   detailed = [[
    Library for Lua for reading dhall configuration files using the `serde-dhall` rust implementation
   ]],
   homepage = "https://github.com/abhashs/dhall-lua",
   license = "MIT"
}

dependencies = {
    "lua >= 5.2",
    "luarocks-build-rust-mlua",
}

build = {
   type = "rust-mlua",
   modules = {
       "dhall_lua"
   }
}
