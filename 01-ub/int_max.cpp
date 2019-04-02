// This demonstrates that integer overflow is undefined (but unsigned integers
// wrap, which is defined). You will likely only observe the behavior if you
// compile in release mode.

#include <climits>
#include <iostream>

using namespace std;

bool is_int_max(int x)
{
    return x + 1 < x;
}

void test(int x)
{
    cout << x << (is_int_max(x)? " is INT_MAX\n" : " isn't INT_MAX\n");
}

int main()
{
    test(3);
}
