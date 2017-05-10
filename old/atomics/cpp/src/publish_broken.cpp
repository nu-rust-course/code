#include "Run_example.h"

#include <iostream>

struct Publish_example
{
    int data{0};
    bool ready{false};

    int result;

    void left()
    {
        data = 10;
        ready = true;
    }

    void right()
    {
        while (!ready) {}
        result = data;
    }

    bool is_valid()
    {
        return result == 10;
    }

    void fmt(std::ostream& os) const
    {
        os << "result: " << result;
    }
};

int main()
{
    std::cout << Run_example<Publish_example>() << '\n';
}
