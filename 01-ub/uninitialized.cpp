
#include <iostream>

int f(bool init)
{
    int x;

    if (init) x = 4;

    return x;
}

int main()
{
    std::cout << f(false) << "\n";
}