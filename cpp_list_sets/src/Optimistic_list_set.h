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
    using guard_t = std::lock_guard<std::mutex>;
    using super = N_lock_list_set<T>;
    using typename super::Node;
    using typename super::link_t;
    using super::link_;

    using const_link_t = std::shared_ptr<const Node>;

    bool validate(const_link_t prev, const_link_t curr) const
    {
        for (const_link_t node = link_; !node->is_last(); node = node->next)
            if (node == prev) return node->next == curr;

        return false;
    }

public:
    virtual bool member(const T& key) const override
    {
        for (;;) {
            link_t prev = super::find_predecessor(key);
            guard_t g1{prev->lock};
            guard_t g2{prev->next->lock};

            if (validate(prev, prev->next))
                return super::matches(*prev, key);
        }
    }

    virtual bool remove(const T& key) override
    {
        for (;;) {
            link_t prev = super::find_predecessor(key);
            guard_t g1{prev->lock};
            guard_t g2{prev->next->lock};

            if (validate(prev, prev->next)) {
                if (!super::matches(*prev, key)) return false;

                prev->next = prev->next->next;
                return true;
            }
        }
    }

    virtual bool insert(T key) override
    {
        for (;;) {
            link_t prev = super::find_predecessor(key);
            guard_t g1{prev->lock};
            guard_t g2{prev->next->lock};

            if (validate(prev, prev->next)) {
                if (super::matches(*prev, key)) return false;

                link_t new_node = std::make_shared<Node>();
                new_node->element = std::move(key);
                new_node->next    = prev->next;
                prev->next        = new_node;

                return true;
            }
        }
    }
};
