#include <iostream>
#include <vector>

void double_repeat(std::vector<int>& v)
{
    for (int& each: v) {
        each *= 2;
        v.push_back(each);
    }
}

template<class T>
void print_vec(const std::vector<T>& v)
{
    std::cout << "{ ";
    for (const T& each: v) std::cout << each << ", ";
    std::cout << "}\n";
}

int main ()
{
    std::vector<int> v{1, 2, 3, 4, 5};
    v.reserve(10); // comment out this line for different behavior
    double_repeat(v);
    print_vec(v);
}