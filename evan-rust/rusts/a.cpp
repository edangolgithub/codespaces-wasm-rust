#include <iostream>

int* make_array() {
    int* p = new int[3]{1,2,3};
    delete[] p;        // freed
    return p;          // returning freed pointer (BUG)
}

int main() {
    int* ptr = make_array();
    std::cout << ptr[0] << std::endl; // ❌ undefined behavior
}
