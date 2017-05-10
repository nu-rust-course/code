#pragma once

#include "Set_base.h"

#include <memory>
#include <mutex>

#undef Set
#define Set Lazy_list_set

template <typename T>
class Lazy_list_set : public Set_base<T>
{
protected:
    struct Node;
    using guard_t      = std::lock_guard<std::mutex>;
    using link_t       = std::shared_ptr<Node>;
    using const_link_t = std::shared_ptr<const Node>;

    struct Node : Node_base<T> {
        T          element;
        link_t     next;
        std::mutex lock;
        bool       marked = false;

        const T& get_element() const { return element; }
        const Node* get_next() const { return &*next; }
    };

    // Head of the list
    link_t link_;

    // Finds the predecessor node of the first node whose element is not less
    // than `key`. That is, if `key` is in the list then it will be found in
    // the result's successor node, and if `key` is not in the list then it
    // belongs between the result and its successor.
    virtual link_t find_predecessor(const T& key) const
    {
        link_t ptr = link_;

        while (!ptr->next->is_last() && key > ptr->next->element) {
            ptr = ptr->next;
        }

        return ptr;
    }

    bool validate(const_link_t prev, const_link_t curr) const
    {
        return prev->next == curr && !prev->marked && !curr->marked;
    }

    static bool matches(const Node& prev, const T& key)
    {
        return !prev.next->is_last()
               && !prev.next->marked
               && prev.next->element == key;
    }

public:
    Lazy_list_set()
    {
        link_ = std::make_shared<Node>();
        link_->next = std::make_shared<Node>();
    }

    virtual bool member(const T& key) const override
    {
        link_t prev = find_predecessor(key);
        return matches(*prev, key);
    }

    virtual bool remove(const T& key) override
    {
        for (;;) {
            link_t prev = find_predecessor(key);
            guard_t g1{prev->lock};
            guard_t g2{prev->next->lock};

            if (validate(prev, prev->next)) {
                if (! matches(*prev, key)) return false;

                prev->next->marked = true;
                prev->next = prev->next->next;
                return true;
            }
        }
    }

    virtual bool insert(T key) override
    {
        for (;;) {
            link_t prev = find_predecessor(key);
            guard_t g1{prev->lock};
            guard_t g2{prev->next->lock};

            if (validate(prev, prev->next)) {
                if (matches(*prev, key)) return false;

                link_t new_node = std::make_shared<Node>();
                new_node->element = std::move(key);
                new_node->next    = prev->next;
                prev->next        = new_node;

                return true;
            }
        }
    }

protected:
    virtual const Node* head() const override
    {
        return &*link_;
    }
};
