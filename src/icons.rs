use ansi_term::Color;
use crate::hash;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::Path,
};

static EXT_ICON_MAP: Lazy<HashMap<OsString, String>> = Lazy::new(|| {
    hash!(
        OsString::from("ai")            => col(185, "\u{e7b4}"),   // 
        OsString::from("awk")           => col(59, "\u{e795}"),    // 
        OsString::from("bash")          => col(113, "\u{e795}"),   // 
        OsString::from("bat")           => col(154, "\u{e615}"),   // 
        OsString::from("bmp")           => col(140, "\u{e60d}"),   // 
        OsString::from("cbl")           => col(25, "\u{2699}"),    // ⚙
        OsString::from("c++")           => col(204, "\u{e61d}"),   // 
        OsString::from("c")             => col(75, "\u{e61e}"),    // 
        OsString::from("cc")            => col(204, "\u{e61d}"),   // 
        OsString::from("cfg")           => col(231, "\u{e7a3}"),   // 
        OsString::from("cljc")          => col(107, "\u{e768}"),   // 
        OsString::from("clj")           => col(107, "\u{e768}"),   // 
        OsString::from("cljd")          => col(67, "\u{e76a}"),    // 
        OsString::from("cljs")          => col(67, "\u{e76a}"),    // 
        OsString::from("cmake")         => col(66, "\u{e615}"),    // 
        OsString::from("cob")           => col(25, "\u{2699}"),    // ⚙
        OsString::from("cobol")         => col(25, "\u{2699}"),    // ⚙
        OsString::from("coffee")        => col(185, "\u{e61b}"),   // 
        OsString::from("conf")          => col(66, "\u{e615}"),    // 
        OsString::from("config.ru")     => col(52, "\u{e791}"),    // 
        OsString::from("cp")            => col(67, "\u{e61d}"),    // 
        OsString::from("cpp")           => col(67, "\u{e61d}"),    // 
        OsString::from("cpy")           => col(25, "\u{2699}"),    // ⚙
        OsString::from("cr")            => col(16, "\u{e24f}"),    // 
        OsString::from("cs")            => col(58, "\u{f81a}"),    // 
        OsString::from("csh")           => col(59, "\u{e795}"),    // 
        OsString::from("cson")          => col(185, "\u{e60b}"),   // 
        OsString::from("css")           => col(39, "\u{e749}"),    // 
        OsString::from("csv")           => col(113, "\u{f718}"),   // 
        OsString::from("cxx")           => col(67, "\u{e61d}"),    // 
        OsString::from("dart")          => col(25, "\u{e798}"),    // 
        OsString::from("db")            => col(188, "\u{e706}"),   // 
        OsString::from("d")             => col(64, "\u{e7af}"),    // 
        OsString::from("desktop")       => col(60, "\u{f108}"),    // 
        OsString::from("diff")          => col(59, "\u{e728}"),    // 
        OsString::from("doc")           => col(25, "\u{f72b}"),    // 
        OsString::from("drl")           => col(217, "\u{e28c}"),   // 
        OsString::from("dropbox")       => col(27, "\u{e707}"),    // 
        OsString::from("dump")          => col(188, "\u{e706}"),   // 
        OsString::from("edn")           => col(67, "\u{e76a}"),    // 
        OsString::from("eex")           => col(140, "\u{e62d}"),   // 
        OsString::from("ejs")           => col(185, "\u{e60e}"),   // 
        OsString::from("elm")           => col(67, "\u{e62c}"),    // 
        OsString::from("epp")           => col(255, "\u{e631}"),   // 
        OsString::from("erb")           => col(52, "\u{e60e}"),    // 
        OsString::from("erl")           => col(132, "\u{e7b1}"),   // 
        OsString::from("ex")            => col(140, "\u{e62d}"),   // 
        OsString::from("exs")           => col(140, "\u{e62d}"),   // 
        OsString::from("f#")            => col(67, "\u{e7a7}"),    // 
        OsString::from("fish")          => col(59, "\u{e795}"),    // 
        OsString::from("fnl")           => col(230, "\u{1f31c}"),  // 🌜
        OsString::from("fs")            => col(67, "\u{e7a7}"),    // 
        OsString::from("fsi")           => col(67, "\u{e7a7}"),    // 
        OsString::from("fsscript")      => col(67, "\u{e7a7}"),    // 
        OsString::from("fsx")           => col(67, "\u{e7a7}"),    // 
        OsString::from("GNUmakefile")   => col(66, "\u{e779}"),    // 
        OsString::from("gd")            => col(66, "\u{e615}"),    // 
        OsString::from("gemspec")       => col(52, "\u{e791}"),    // 
        OsString::from("gif")           => col(140, "\u{e60d}"),   // 
        OsString::from("git")           => col(202, "\u{e702}"),   // 
        OsString::from("glb")           => col(215, "\u{f1b2}"),   // 
        OsString::from("go")            => col(67, "\u{e627}"),    // 
        OsString::from("godot")         => col(66, "\u{e7a3}"),    // 
        OsString::from("gql")           => col(199, "\u{f20e}"),   // 
        OsString::from("graphql")       => col(199, "\u{f20e}"),   // 
        OsString::from("haml")          => col(188, "\u{e60e}"),   // 
        OsString::from("hbs")           => col(208, "\u{e60f}"),   // 
        OsString::from("h")             => col(140, "\u{f0fd}"),   // 
        OsString::from("heex")          => col(140, "\u{e62d}"),   // 
        OsString::from("hh")            => col(140, "\u{f0fd}"),   // 
        OsString::from("hpp")           => col(140, "\u{f0fd}"),   // 
        OsString::from("hrl")           => col(132, "\u{e7b1}"),   // 
        OsString::from("hs")            => col(140, "\u{e61f}"),   // 
        OsString::from("htm")           => col(166, "\u{e60e}"),   // 
        OsString::from("html")          => col(202, "\u{e736}"),   // 
        OsString::from("hxx")           => col(140, "\u{f0fd}"),   // 
        OsString::from("ico")           => col(185, "\u{e60d}"),   // 
        OsString::from("import")        => col(231, "\u{f0c6}"),   // 
        OsString::from("ini")           => col(66, "\u{e615}"),    // 
        OsString::from("java")          => col(167, "\u{e738}"),   // 
        OsString::from("jl")            => col(133, "\u{e624}"),   // 
        OsString::from("jpeg")          => col(140, "\u{e60d}"),   // 
        OsString::from("jpg")           => col(140, "\u{e60d}"),   // 
        OsString::from("js")            => col(185, "\u{e60c}"),   // 
        OsString::from("json5")         => col(185, "\u{fb25}"),   // ﬥ
        OsString::from("json")          => col(185, "\u{e60b}"),   // 
        OsString::from("jsx")           => col(67, "\u{e625}"),    // 
        OsString::from("ksh")           => col(59, "\u{e795}"),    // 
        OsString::from("kt")            => col(99, "\u{e634}"),    // 
        OsString::from("kts")           => col(99, "\u{e634}"),    // 
        OsString::from("leex")          => col(140, "\u{e62d}"),   // 
        OsString::from("less")          => col(60, "\u{e614}"),    // 
        OsString::from("lhs")           => col(140, "\u{e61f}"),   // 
        OsString::from("license")       => col(185, "\u{e60a}"),   // 
        OsString::from("lock")          => col(250, "\u{f13e}"),   // 
        OsString::from("log")           => col(255, "\u{f831}"),   // 
        OsString::from("lua")           => col(74, "\u{e620}"),    // 
        OsString::from("luau")          => col(74, "\u{e620}"),    // 
        OsString::from("makefile")      => col(66, "\u{e779}"),    // 
        OsString::from("markdown")      => col(67, "\u{e609}"),    // 
        OsString::from("Makefile")      => col(66, "\u{e779}"),    // 
        OsString::from("material")      => col(132, "\u{f7f4}"),   // 
        OsString::from("md")            => col(255, "\u{f48a}"),   // 
        OsString::from("mdx")           => col(67, "\u{f48a}"),    // 
        OsString::from("mint")          => col(108, "\u{f829}"),   // 
        OsString::from("mjs")           => col(221, "\u{e60c}"),   // 
        OsString::from("mk")            => col(66, "\u{e779}"),    // 
        OsString::from("ml")            => col(173, "\u{3bb}"),    // λ
        OsString::from("mli")           => col(173, "\u{3bb}"),    // λ
        OsString::from("mo")            => col(99, "\u{221e}"),    // ∞
        OsString::from("mustache")      => col(173, "\u{e60f}"),   // 
        OsString::from("nim")           => col(220, "\u{1f451}"),  // 👑
        OsString::from("nix")           => col(110, "\u{f313}"),   // 
        OsString::from("opus")          => col(208, "\u{f722}"),   // 
        OsString::from("otf")           => col(231, "\u{f031}"),   // 
        OsString::from("pck")           => col(66, "\u{f487}"),    // 
        OsString::from("pdf")           => col(124, "\u{f724}"),   // 
        OsString::from("php")           => col(140, "\u{e608}"),   // 
        OsString::from("pl")            => col(67, "\u{e769}"),    // 
        OsString::from("pm")            => col(67, "\u{e769}"),    // 
        OsString::from("png")           => col(140, "\u{e60d}"),   // 
        OsString::from("pp")            => col(255, "\u{e631}"),   // 
        OsString::from("ppt")           => col(167, "\u{f726}"),   // 
        OsString::from("prisma")        => col(255, "\u{5351}"),   // 卑
        OsString::from("pro")           => col(179, "\u{e7a1}"),   // 
        OsString::from("ps1")           => col(69, "\u{f0a0a}"),   // 󰨊
        OsString::from("psb")           => col(67, "\u{e7b8}"),    // 
        OsString::from("psd1")          => col(105, "\u{f0a0a}"),  // 󰨊
        OsString::from("psd")           => col(67, "\u{e7b8}"),    // 
        OsString::from("psm1")          => col(105, "\u{f0a0a}"),  // 󰨊
        OsString::from("pyc")           => col(67, "\u{e606}"),    // 
        OsString::from("py")            => col(61, "\u{e606}"),    // 
        OsString::from("pyd")           => col(67, "\u{e606}"),    // 
        OsString::from("pyo")           => col(67, "\u{e606}"),    // 
        OsString::from("query")         => col(154, "\u{e21c}"),   // 
        OsString::from("rake")          => col(52, "\u{e791}"),    // 
        OsString::from("rb")            => col(52, "\u{e791}"),    // 
        OsString::from("r")             => col(65, "\u{fcd2}"),    // ﳒ
        OsString::from("rlib")          => col(180, "\u{e7a8}"),   // 
        OsString::from("rmd")           => col(67, "\u{e609}"),    // 
        OsString::from("rproj")         => col(65, "\u{9276}"),    // 鉶
        OsString::from("rs")            => col(180, "\u{e7a8}"),   // 
        OsString::from("rss")           => col(215, "\u{e619}"),   // 
        OsString::from("sass")          => col(204, "\u{e603}"),   // 
        OsString::from("sbt")           => col(167, "\u{e737}"),   // 
        OsString::from("scala")         => col(167, "\u{e737}"),   // 
        OsString::from("scm")           => col(16, "\u{fb26}"),    // ﬦ
        OsString::from("scss")          => col(204, "\u{e603}"),   // 
        OsString::from("sh")            => col(59, "\u{e795}"),    // 
        OsString::from("sig")           => col(173, "\u{3bb}"),    // λ
        OsString::from("slim")          => col(166, "\u{e60e}"),   // 
        OsString::from("sln")           => col(98, "\u{e70c}"),    // 
        OsString::from("sml")           => col(173, "\u{3bb}"),    // λ
        OsString::from("sol")           => col(67, "\u{fcb9}"),    // ﲹ
        OsString::from("sql")           => col(188, "\u{e706}"),   // 
        OsString::from("sqlite3")       => col(188, "\u{e706}"),   // 
        OsString::from("sqlite")        => col(188, "\u{e706}"),   // 
        OsString::from("styl")          => col(107, "\u{e600}"),   // 
        OsString::from("sublime")       => col(98, "\u{e7aa}"),    // 
        OsString::from("suo")           => col(98, "\u{e70c}"),    // 
        OsString::from("sv")            => col(29, "\u{f85a}"),    // 
        OsString::from("svelte")        => col(202, "\u{f260}"),   // 
        OsString::from("svg")           => col(215, "\u{fc1f}"),   // ﰟ
        OsString::from("svh")           => col(29, "\u{f85a}"),    // 
        OsString::from("swift")         => col(173, "\u{e755}"),   // 
        OsString::from("tbc")           => col(67, "\u{fbd1}"),    // ﯑
        OsString::from("t")             => col(67, "\u{e769}"),    // 
        OsString::from("tcl")           => col(67, "\u{fbd1}"),    // ﯑
        OsString::from("terminal")      => col(71, "\u{f489}"),    // 
        OsString::from("test.js")       => col(173, "\u{e60c}"),   // 
        OsString::from("tex")           => col(58, "\u{fb68}"),    // ﭨ
        OsString::from("tf")            => col(57, "\u{e2a6}"),    // 
        OsString::from("tfvars")        => col(57, "\u{f15b}"),    // 
        OsString::from("toml")          => col(66, "\u{e615}"),    // 
        OsString::from("tres")          => col(185, "\u{e706}"),   // 
        OsString::from("ts")            => col(67, "\u{e628}"),    // 
        OsString::from("tscn")          => col(140, "\u{f880}"),   // 
        OsString::from("tsx")           => col(67, "\u{e7ba}"),    // 
        OsString::from("twig")          => col(107, "\u{e61c}"),   // 
        OsString::from("txt")           => col(113, "\u{f718}"),   // 
        OsString::from("vala")          => col(5, "\u{e69e}"),     // 
        OsString::from("v")             => col(29, "\u{f85a}"),    // 
        OsString::from("vh")            => col(29, "\u{f85a}"),    // 
        OsString::from("vhd")           => col(29, "\u{f85a}"),    // 
        OsString::from("vhdl")          => col(29, "\u{f85a}"),    // 
        OsString::from("vim")           => col(29, "\u{e62b}"),    // 
        OsString::from("vue")           => col(107, "\u{fd42}"),   // ﵂
        OsString::from("wasm")          => col(99, "\u{e6a1}"),    // 
        OsString::from("webmanifest")   => col(221, "\u{e60b}"),   // 
        OsString::from("webpack")       => col(67, "\u{fc29}"),    // ﰩ
        OsString::from("webp")          => col(140, "\u{e60d}"),   // 
        OsString::from("xcplayground")  => col(173, "\u{e755}"),   // 
        OsString::from("xls")           => col(23, "\u{f71a}"),    // 
        OsString::from("xml")           => col(173, "\u{8b39}"),   // 謹
        OsString::from("xul")           => col(173, "\u{e745}"),   // 
        OsString::from("yaml")          => col(66, "\u{e615}"),    // 
        OsString::from("yml")           => col(66, "\u{e615}"),    // 
        OsString::from("zig")           => col(208, "\u{f0e7}"),   // 
        OsString::from("zsh")           => col(113, "\u{e795}")    // 
    )
});

static DEFAULT_ICON: Lazy<String> = Lazy::new(|| col(66, "\u{f15b}"));

pub fn icon(path: &Path) -> &str {
    path.extension()
        .map(icon_from_ext)
        .unwrap_or_else(get_default_icon)
}

fn get_default_icon<'a>() -> &'a str {
    DEFAULT_ICON.as_str()
}

/// Reference: https://github.com/nvim-tree/nvim-web-devicons/blob/master/lua/nvim-web-devicons.lua
fn icon_from_ext(ext: &OsStr) -> &str {
    EXT_ICON_MAP.get(ext)
        .map(String::as_str)
        .unwrap_or_else(get_default_icon)
}

fn col(num: u8, code: &str) -> String {
    Color::Fixed(num).paint(code).to_string()
}
