// Searches for sequentially inconsistent behavior.

#include "Run_example.h"

#include <iostream>

struct Data_race_example
{
    int x{0};
    int y{0};

    int l = -1;
    int r = -1;

    void left()
    {
        x = 1;
        l = y;
    }

    void right()
    {
        y = 1;
        r = x;
    }

    bool is_valid() const
    {
        return (l == 0 && r == 1) ||
               (l == 1 && r == 0) ||
               (l == 1 && r == 1);
    }

    void fmt(std::ostream& os) const
    {
        os << "l == " << l << " && r == " << r;
    }
};

int main()
{
    std::cout << Run_example<Data_race_example>() << '\n';
}
