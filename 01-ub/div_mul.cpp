#include <iostream>
#include <climits>

int identity(int x, int y)
{
    return x * y / y;
}

void print_identity(int x, int y)
{
    std::cout << "identity(" << x << ", " << y << ") == "
              << identity(x, y) << '\n';
}

int main()
{
    print_identity(7, 5);
    print_identity(7, INT_MAX);
    print_identity(7, 0);
}