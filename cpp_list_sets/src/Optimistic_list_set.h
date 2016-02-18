#pragma once

#include "N_lock_list_set.h"

#include <iostream>
#include <memory>
#include <mutex>
#include <tuple>

#undef Set
#define Set Optimistic_list_set

template <typename T>
class Optimistic_list_set : public N_lock_list_set<T>
{
protected:
    using super   = N_lock_list_set<T>;
    using Node    = typename super::Node;
    using link_t  = typename super::link_t;
    using guard_t = typename super::guard_t;

    link_t& link_ = super::link_;

    bool validate(const Node* pred, const Node* succ)
    {
        for (Node* curr = &*link_; !curr->is_last(); curr = &*curr->next)
            if (curr == pred) return (&*curr->next == succ);

        return false;
    }

public:
    virtual bool remove(const T& key) override
    {
        for (;;) {
            auto& pred = super::find_predecessor(key);
            guard_t g1{pred.lock};
            guard_t g2{pred.next->lock};

            if (validate(&pred, &*pred.next)) {
                if (pred.next->is_last() || pred.next->element != key)
                    return false;

                pred.next = std::move(pred.next->next);
                return true;
            }
        }
    }

    virtual bool insert(T key) override
    {
        for (;;) {
            auto& pred = super::find_predecessor(key);
            guard_t g1{pred.lock};
            guard_t g2{pred.next->lock};

            if (validate(&pred, &*pred.next)) {
                if (!pred.next->is_last() && pred.next->element == key)
                    return false;

                std::unique_ptr<Node> new_node{new Node{}};
                new_node->element = key;
                new_node->next    = std::move(pred.next);
                pred.next         = std::move(new_node);

                return true;
            }
        }
    }
};
