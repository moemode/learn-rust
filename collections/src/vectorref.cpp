#include <iostream>
#include <vector>

int main() {
    std::vector<int> numbers;
    numbers.reserve(2);  // Reserve space for 3 elements
    numbers.push_back(10);
    numbers.push_back(20);
    // Get a reference to the second element
    int& secondElement = numbers[1];

    numbers.push_back(30);

    // Attempt to access the second element through the reference
    std::cout << "Second element: " << secondElement << std::endl;

    return 0;
}