// Simple mutex depends on seq cst

#include "Run_example.h"

#include <atomic>
#include <iostream>

const std::memory_order order = std::memory_order_seq_cst;

struct Mutex_example
{
    std::atomic<int> x{0};
    std::atomic<int> y{0};

    int counter{0};

    void left()
    {
        x.store(1, order);
        if (y.load(order) == 0) counter += 1;
    }

    void right()
    {
        y.store(1, order);
        if (x.load(order) == 0) counter += 2;
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
    std::cout << Run_example<Mutex_example>(100'000) << '\n';
}
