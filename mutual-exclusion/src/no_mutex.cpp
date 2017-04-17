#include <iostream>
#include <thread>

class Counter {
public:
    int get() const {
        return count_;
    }

    void inc() {
        ++count_;
    }

private:
    int count_ = 0;
};

// int main()
// {
//     Counter c;

//     std::thread t1([&]() { c.inc(); });
//     std::thread t2([&]() { c.inc(); });

//     t1.join();
//     t2.join();

//     std::cout << c.get() << '\n';
// }
