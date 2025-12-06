#include <emscripten.h>

// Export the function with C linkage to avoid name mangling
extern "C" {
    EMSCRIPTEN_KEEPALIVE
    int add(int a, int b) {
        return a + b;
    }
    
    // Optional: A function to show it works
    EMSCRIPTEN_KEEPALIVE
    const char* getMessage() {
        return "Hello from WebAssembly!";
    }
}