#include <UnitTest++/UnitTest++.h>

#include <iostream>
#include <thread>

using namespace std;

class Counter {
public:
    int get_and_inc() {
        int old = count_;
        count_ = old + 1;
        return old;
    }

private:
    int count_ = 0;
};

TEST(Counter_test)
{
    Counter c;

    thread t1([&]() { c.get_and_inc(); });
    thread t2([&]() { c.get_and_inc(); });

    t1.join();
    t2.join();

    // This won't necessarily pass!
    CHECK_EQUAL(2, c.get_and_inc());
}
