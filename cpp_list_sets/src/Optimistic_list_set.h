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
    using super = N_lock_list_set<T>;
    using typename super::Node;
    using typename super::guard_t;
    using super::link_;

    bool validate(const Node* prev, const Node* curr) const
    {
        for (const Node* node = &*link_; !node->is_last(); node = &*node->next)
            if (node == prev) return (&*node->next == curr);

        return false;
    }

public:
    virtual bool member(const T& key) const override
    {
        for (;;) {
            auto& prev = super::find_predecessor(key);
            guard_t g1{prev.lock};
            guard_t g2{prev.next->lock};

            if (validate(&prev, &*prev.next))
                return super::matches(prev, key);
        }
    }

    virtual bool remove(const T& key) override
    {
        for (;;) {
            auto& prev = super::find_predecessor(key);
            guard_t g1{prev.lock};
            guard_t g2{prev.next->lock};

            if (validate(&prev, &*prev.next)) {
                if (!super::matches(prev, key)) return false;

                prev.next = std::move(prev.next->next);
                return true;
            }
        }
    }

    virtual bool insert(T key) override
    {
        for (;;) {
            auto& prev = super::find_predecessor(key);
            guard_t g1{prev.lock};
            guard_t g2{prev.next->lock};

            if (validate(&prev, &*prev.next)) {
                if (super::matches(prev, key)) return false;

                std::unique_ptr<Node> new_node = std::make_unique<Node>();
                new_node->element = std::move(key);
                new_node->next    = std::move(prev.next);
                prev.next         = std::move(new_node);

                return true;
            }
        }
    }
};
