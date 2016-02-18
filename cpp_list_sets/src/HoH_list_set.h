#pragma once

#include "N_lock_list_set.h"

class HoH_list_set : public N_lock_list_set
{
    // Like `find_predecessor`, finds the predecessor node of the first node
    // whose element is not less than `key`. Performs hand-over-hand
    // locking, with the postcondition that the mutexes on both the result node
    // and its successor are held, guaranteeing that neither is deleted.
    // Returns a triple of the reference to the predecessor node, the guard
    // for that node, and the guard for its successor. Destruction of the
    // guards will unlock the mutexes.
    virtual std::tuple<Node*, guard_t, guard_t>
    find_predecessor_locking(const T& key) const override
    {
        Node* ptr = &*link_;
        guard_t curr{ptr->lock};
        guard_t next{ptr->next->lock};

        while (!ptr->next->is_last() && key > ptr->next->element) {
            curr = std::move(next);
            ptr  = &*ptr->next;
            next = guard_t{ptr->next->lock};
        }

        return {ptr, std::move(curr), std::move(next)};
    }
};


#endif //LISTSET_HOH_LIST_SET_H
