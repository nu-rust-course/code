// Simple mutex depends on seq cst

#include "Run_example.h"

#include <iostream>

struct Mutex_example
{
    int x{0};
    int y{0};

    int counter{0};

    void left()
    {
        x = 1;
        if (y == 0) counter += 1;
    }

    void right()
    {
        y = 1;
        if (x == 0) counter += 2;
    }

    bool is_valid()
    {
        return counter == 0 || counter == 1 || counter == 2;
    }

    void fmt(std::ostream& os) const
    {
        os << "counter: " << counter;
    }
};

int main()
{
    std::cout << Run_example<Mutex_example>() << '\n';
}
