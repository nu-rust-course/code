#include "Run_example.h"
#include <iostream>

struct Example_example
{
    int x = 0;
    int y = 0;

    void left()
    {
        ++x;
    }

    void right()
    {
        ++y;
    }

    bool is_valid()
    {
        return x == 1 && y == 1;
    }

    void fmt(std::ostream& os)
    {
        os << "x == " << x << " && y == " << y;
    }
};

int main()
{
    std::cout << Run_example<Example_example>(100'000) << '\n';
}
