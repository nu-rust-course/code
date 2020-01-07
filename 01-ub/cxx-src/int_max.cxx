// This demonstates that integer overflow is undefined (but unsigned integers
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

bool is_uint_max(unsigned int x)
{
    return x + 1u < x;
}

void test(unsigned int x)
{
    cout << x << (is_uint_max(x)? " is UINT_MAX\n" : " isn't UINT_MAX\n");
}

int main()
{
    test(INT_MAX);
    test(UINT_MAX);
}
