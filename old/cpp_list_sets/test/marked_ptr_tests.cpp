#include "marked_ptr.h"

#include <vector>
#include <UnitTest++/UnitTest++.h>

TEST(Construct)
{
    marked_ptr<int> mp;
}

TEST(FillAndChange)
{
    int a = 8, b = 9;

    marked_ptr<int> mp{&a, false};

    CHECK_EQUAL(&a, mp.pointer());
    CHECK_EQUAL(a, *mp);
    CHECK_EQUAL(false, mp.mark());
    CHECK_EQUAL(false, bool{mp});

    mp.set_mark(true);

    CHECK_EQUAL(&a, mp.pointer());
    CHECK_EQUAL(true, mp.mark());

    mp.pointer(&b);

    CHECK_EQUAL(&b, mp.pointer());
    CHECK_EQUAL(true, mp.mark());
}

TEST(OperatorArrow)
{
    std::vector<int> v{1, 2, 3};

    marked_ptr<std::vector<int>> mp{&v, true};

    CHECK_EQUAL(3, mp->size());
}

TEST(ConstructAndAssignAtomic)
{
    int                          a = 9;
    std::atomic<marked_ptr<int>> amp{&a, true};

    CHECK_EQUAL(a,  *amp);
    CHECK_EQUAL(&a, &*amp);
}
