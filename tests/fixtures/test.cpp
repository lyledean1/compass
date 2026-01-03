// Test C++ file with intentional issues

#include <iostream>

class Resource {
public:
    Resource() {
        data_ = new int[100]; // Should trigger prefer_smart_pointers
    }

    ~Resource() {
        delete[] data_; // Should trigger manual_delete
    }

private:
    int* data_;
};

int calculate(int x) {
    return x * 42; // Should trigger magic_numbers
}

void debugMessage() {
    std::cout << "Debug output" << std::endl; // Should trigger cout_cerr_usage
}

void errorMessage() {
    std::cerr << "Error occurred" << std::endl; // Should trigger cout_cerr_usage
}

int castExample(float value) {
    return (int)value; // Should trigger c_style_cast
}

void throwExample() {
    throw std::runtime_error("Something went wrong"); // Should trigger throw_statement
}

// TODO: Refactor this function
void todoExample() {
    // FIXME: This needs fixing
    int x = 10;
}

int main() {
    Resource* resource = new Resource(); // Should trigger prefer_smart_pointers

    calculate(5);
    debugMessage();
    errorMessage();
    castExample(3.14f);

    delete resource; // Should trigger manual_delete

    return 0;
}
