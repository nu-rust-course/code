// Searches for sequentially inconsistent behavior.

#include "Run_example.h"

#include <atomic>
#include <iostream>

const std::memory_order order = std::memory_order_seq_cst;

struct Data_race_example
{
    std::atomic<int> x{0};
    std::atomic<int> y{0};

    int l = -1;
    int r = -1;

    void left()
    {
        x.store(1, order);
        l = y.load(order);
    }

    void right()
    {
        y.store(1, order);
        r = x.load(order);
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
    std::cout << Run_example<Data_race_example>(100'000) << '\n';
}
