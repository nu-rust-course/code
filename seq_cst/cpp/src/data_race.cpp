// C++ rendering of seq_cst.cpp
// Searches for sequentially inconsistent behavior.

#include <iostream>
#include <thread>

class Example
{
    int x = 0;
    int y = 0;
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

public:
    Example()
    {
        std::thread tl([&]() { left(); });
        std::thread tr([&]() { right(); });

        tl.join();
        tr.join();
    }

    static Example search()
    {
        for (;;) {
            Example e;
            if (!e.is_valid()) return e;
        }
    }

    std::ostream& fmt(std::ostream& o) const
    {
        return o << "l == " << l << " && r == " << r;
    }
};

std::ostream& operator<<(std::ostream& o, const Example& e)
{
    return e.fmt(o);
}

int main()
{
    std::cout << Example::search() << '\n';
}
