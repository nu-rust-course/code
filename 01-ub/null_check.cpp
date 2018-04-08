// This one behaves differently in debug and release modes as well. The
// release mode behavior is surprising. Why does it do that?

#include <iostream>

void zero_ptr(int* p) {
    int dead = *p;
    if (p == nullptr) return;
    std::cerr << "howdy!\n";
    *p = 0;
}

int main()
{
    int* px = nullptr;
    zero_ptr(px);
}