extern crate cc;
extern crate cmake;
//extern crate pkg_config

use std::env;
// use std::ascii::AsciiExt;

// use std::ffi::OsString;
// use std::fs;
// use std::path::{PathBuf, Path, Component, Prefix};
// use std::process::Command;
// use std::io::ErrorKind;

fn main() {
    // let target = env::var("TARGET").unwrap();
    // let host = env::var("HOST").unwrap();
    // let src = env::current_dir().unwrap();
    // let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    //env::set_var("CMAKE_PREFIX_PATH", "/tmp/qt58/lib/cmake/");
    //env::set_var("LD_LIBRARY_PATH", "/home/richard/Qt/5.8/gcc_64/lib");
    //let mut cfg = cmake::Config::new("DOtherSide");
    //cfg.build();

    build_and_link_dotherside();
}

fn build_and_link_dotherside() {
    let mut cfg = cc::Build::new();

    if env::var("TARGET").unwrap().contains("windows") {
        cfg.define("WIN32", None);
    }

    cfg.cpp(true)
       .static_flag(true)
       .flag_if_supported("-std=c++11")
       .warnings(false);
       //.flag_if_supported("-Wall")
       //.flag_if_supported("-Wno-long-long")
       //.flag_if_supported("-Wno-unused-variable");
       //.flag_if_supported("-pedantic");

    cfg.include("DOtherSide/lib/include")
       .include("DOtherSide/lib/include/Qt")
       .include("/tmp/qt58/include/")
       .file("DOtherSide/lib/src/DOtherSide.cpp")
       .file("DOtherSide/lib/src/DOtherSideTypesCpp.cpp")
       .file("DOtherSide/lib/src/DosQAbstractItemModel.cpp")
       .file("DOtherSide/lib/src/DosQDeclarative.cpp")
       .file("DOtherSide/lib/src/DosQMetaObject.cpp")
       .file("DOtherSide/lib/src/DosQObject.cpp")
       .file("DOtherSide/lib/src/DosQObjectImpl.cpp")
       .file("DOtherSide/lib/src/DosQQuickImageProvider.cpp")
       .file("DOtherSide/lib/src/OnSlotExecutedHandler.cpp")
       .compile("dotherside");
}

fn build_and_link_qt() {

}