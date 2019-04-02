#include <iostream>

int f(int i)
{
    int array[3] = {0};
    array[i] = 12;
    return array[i] + array[0];
}

int main()
{
    for (int i = 0; i < 8; ++i)
        std::cout << f(i) << '\n';
}