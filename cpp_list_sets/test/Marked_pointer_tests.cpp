#include "Marked_pointer.h"

#include <vector>
#include <UnitTest++/UnitTest++.h>

TEST(Construct)
{
    Marked_pointer<int> mp;
}

TEST(FillAndChange)
{
    int a = 8, b = 9;

    Marked_pointer<int> mp{&a, false};

    CHECK_EQUAL(&a, mp.pointer());
    CHECK_EQUAL(a, *mp);
    CHECK_EQUAL(false, mp.mark());
    CHECK_EQUAL(false, bool{mp});

    mp.mark(true);

    CHECK_EQUAL(&a, mp.pointer());
    CHECK_EQUAL(true, mp.mark());

    mp.pointer(&b);

    CHECK_EQUAL(&b, mp.pointer());
    CHECK_EQUAL(true, mp.mark());
}

TEST(OperatorArrow)
{
    std::vector<int> v{1, 2, 3};

    Marked_pointer<std::vector<int>> mp{&v, true};

    CHECK_EQUAL(3, mp->size());
}

TEST(ConstructAndAssignAtomic)
{
    int                              a = 9;
    std::atomic<Marked_pointer<int>> amp{&a, true};

    CHECK_EQUAL(a,  *amp);
    CHECK_EQUAL(&a, &*amp);
}
