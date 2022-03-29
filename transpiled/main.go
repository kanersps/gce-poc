package main

// #cgo LDFLAGS: -L/Users/kanepetra/gitprojects/loop_plugins/plugin_loader/target/release -lplugin_loader
//
// #include "../plugin_loader/libplugin_loader.h"
import "C"


func main() {
	C.load_plugin(C.CString("Hey!"));
}