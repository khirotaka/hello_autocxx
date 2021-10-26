#include <iostream>
#include <string>


void greet(std::string name) {
    std::cout << "Hello! " << name << std::endl;
}

float add(float a, float b) {
    return a + b;
}


namespace func {
    float mul(float a, float b) {
        return a * b;
    }
}
