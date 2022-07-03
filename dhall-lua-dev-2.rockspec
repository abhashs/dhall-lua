package = "dhall-lua"
version = "dev-2"

source = {
   url = "git+https://github.com/abhashs/dhall-lua.git",
   tag = "0.1.0"
}

description = {
   summary = "Lua bindings for dhall configuration language",
   detailed = [[
    Library for reading Dhall configuration files using the `serde-dhall` Rust implementation
   ]],
   homepage = "https://github.com/abhashs/dhall-lua",
   license = "MIT"
}

dependencies = {
    "lua >= 5.1",
    "luarocks-build-rust-mlua",
}

build = {
   type = "rust-mlua",
   modules = {
       "dhall_lua"
   }
}
