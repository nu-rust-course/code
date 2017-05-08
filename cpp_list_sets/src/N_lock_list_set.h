#pragma once

#include "List_set.h"

#include <iostream>
#include <memory>
#include <mutex>
#include <tuple>

#undef Set
#define Set N_lock_list_set

template <typename T>
class N_lock_list_set : public Set_base<T>
{
protected:
    struct Node;
    using link_t  = std::unique_ptr<Node>;
    using guard_t = std::unique_lock<std::mutex>;

    struct Node : public Node_base<T> {
        T          element;
        link_t     next;
        std::mutex lock;

        const T& get_element() const { return element; }
        const Node* get_next() const { return &*next; }
    };

    // Head of the list
    link_t link_;

    // Finds the predecessor node of the first node whose element is not less
    // than `key`. That is, if `key` is in the list then it will be found in
    // the result's successor node, and if `key` is not in the list then it
    // belongs between the result and its successor.
    virtual Node& find_predecessor(const T& key) const
    {
        Node* ptr = &*link_;

        while (!ptr->next->is_last() && key > ptr->next->element) {
            ptr = &*ptr->next;
        }

        return *ptr;
    }

    // Note: This implementation is faulty. For a correct implementation,
    // see HoH_list_set.h.
    //
    // Like `find_predecessor`, finds the predecessor node of the first node
    // whose element is not less than `key`. Locks each node as it traverses it,
    // with the postcondition that the lock on the returned node is held.
    // This faulty implementation returns a triple of the reference to the
    // predecessor node, the guard for that node, and an empty guard.
    // Destruction of the guard will unlock the mutex.
    virtual std::tuple<Node*, guard_t, guard_t>
    find_predecessor_locking(const T& key) const
    {
        Node* ptr = &*link_;
        guard_t guard{ptr->lock};

        while (!ptr->next->is_last() && key > ptr->next->element) {
            guard = guard_t{ptr->next->lock};
            ptr   = &*ptr->next;
        }

        return {ptr, std::move(guard), guard_t{}};
    }

    bool matches(const Node& prev, const T& key) const
    {
        return !prev.next->is_last() && prev.next->element == key;
    }

public:
    N_lock_list_set()
    {
        link_       = std::make_unique<Node>(); // head sentinel
        link_->next = std::make_unique<Node>(); // tail sentinel
    }

    virtual bool member(const T& key) const override
    {
        Node* prev;
        guard_t g1, g2;
        std::tie(prev, g1, g2) = find_predecessor_locking(key);

        return matches(*prev, key);
    }

    virtual bool remove(const T& key) override
    {
        Node* prev;
        guard_t g1, g2;
        std::tie(prev, g1, g2) = find_predecessor_locking(key);

        if (! matches(*prev, key)) return false;

        prev->next = std::move(prev->next->next);
        return true;
    }

    virtual bool insert(T key) override
    {
        Node* prev;
        guard_t g1, g2;
        std::tie(prev, g1, g2) = find_predecessor_locking(key);

        if (matches(*prev, key)) return false;

        std::unique_ptr<Node> new_node = std::make_unique<Node>();
        new_node->element = std::move(key);
        new_node->next    = std::move(prev->next);
        prev->next        = std::move(new_node);

        return true;
    }

    virtual const Node_base<T>* head() const override
    {
        return &*link_;
    }
};
