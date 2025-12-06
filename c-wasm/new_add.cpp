#include <emscripten/bind.h>

using namespace emscripten;


int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

std::string greet(const std::string& name) {
    return "Hello, " + name + "!";
}

// Automatically exports all bound functions
EMSCRIPTEN_BINDINGS(my_module) {
    function("add", &add);
    function("multiply", &multiply);
    function("greet", &greet);
}

//em++ add.cpp -O3 --bind -o new_add.js
//em++ add_new.cpp -O3 --bind -o new_add.js    
//em++ new_add.cpp -O3 --bind -s EXPORT_ES6=1 -s MODULARIZE=1 -s EXPORT_NAME='createWasmModule' -o new_add.js