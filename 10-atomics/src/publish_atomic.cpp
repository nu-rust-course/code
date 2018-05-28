#include "Run_example.h"

#include <atomic>
#include <iostream>

struct Publish_example
{
    int data{0};
    std::atomic<bool> ready{false};

    int result;

    void left()
    {
        data = 10;
        ready.store(true, std::memory_order_release);
    }

    void right()
    {
        while (!ready.load(std::memory_order_acquire)) {}
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
    std::cout << Run_example<Publish_example>(100'000) << '\n';
}
