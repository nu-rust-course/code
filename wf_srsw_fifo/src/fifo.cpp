#include "fifo.h"
#include <UnitTest++/UnitTest++.h>

TEST(Fifo) {
    Wf_SRSW_FIFO<int, 8> q;

    q.enq(3);
    CHECK_EQUAL(3, q.deq());
}
