#include "Lock_free_list_set.h"

#include <UnitTest++/UnitTest++.h>

TEST(New)
{
    Set<int> empty;
}

TEST(Member)
{
    Set<int> set;

    CHECK_EQUAL(false, set.member(5));
    CHECK_EQUAL(false, set.member(6));
    CHECK_EQUAL(false, set.member(7));
}

TEST(Insert)
{
    Set<int> set;

    CHECK_EQUAL(true, set.insert(5));
    CHECK_EQUAL(false, set.insert(5));
    CHECK_EQUAL(true, set.insert(6));
    CHECK_EQUAL(true, set.insert(7));
}

TEST(Insert_and_member)
{
    Set<int> set;

    set.insert(5);
    set.insert(6);
    set.insert(7);

    CHECK_EQUAL(false, set.member(4));
    CHECK_EQUAL(true,  set.member(5));
    CHECK_EQUAL(true,  set.member(6));
    CHECK_EQUAL(true,  set.member(7));
    CHECK_EQUAL(false, set.member(8));
}

TEST(Remove)
{
    Set<int> set;

    CHECK_EQUAL(false, set.remove(5));
}

TEST(Insert_and_remove)
{
    Set<int> set;

    CHECK_EQUAL(false, set.remove(5));
    CHECK_EQUAL(true, set.insert(5));
    CHECK_EQUAL(false, set.insert(5));
    CHECK_EQUAL(false, set.insert(5));
    CHECK_EQUAL(true, set.remove(5));
    CHECK_EQUAL(false, set.remove(5));
    CHECK_EQUAL(true, set.insert(5));
    CHECK_EQUAL(true, set.remove(5));
    CHECK_EQUAL(true, set.insert(5));
    CHECK_EQUAL(true, set.remove(5));
}

TEST(Remove_regression)
{
    Set<int> set;

    CHECK_EQUAL(true, set.insert(4));
    CHECK_EQUAL(true, set.insert(6));
    CHECK_EQUAL(false, set.remove(5));
}

TEST(Format_to)
{
    Set<int> set;
    std::ostringstream os;

    os << set << ' ';
    set.insert(2);
    os << set << ' ';
    set.insert(4);
    set.insert(3);
    os << set;

    CHECK_EQUAL("{} { 2 } { 2, 3, 4 }", os.str());
}
