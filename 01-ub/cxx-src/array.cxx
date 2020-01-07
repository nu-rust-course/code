#include <iostream>

int f(int i)
{
    // Try changing the size on the next line from 3 to 1.
    int array[3] = {0};
    array[i] = 12;
    return array[i] + array[0];
}

int main()
{
    for (int i = 0; i < 8; ++i)
        std::cout << f(i) << '\n';
}