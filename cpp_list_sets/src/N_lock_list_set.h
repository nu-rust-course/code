#pragma once

#include "Set_base.h"

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

    struct Node {
        T          element;
        link_t     next;
        std::mutex lock;

        bool is_last()
        {
            return next == nullptr;
        }
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

    // Like `find_predecessor`, finds the predecessor node of the first node
    // whose element is not less than `key`. Performs hand-over-hand
    // locking, with the postcondition that the mutexs on both the result node
    // and its successor are held, guaranteeing that neither is deleted.
    // Returns a triple of the reference to the predecessor node, the guard
    // for that node, and the guard for its successor. Destruction of the
    // guards will unlock the mutexes.
    virtual std::tuple<Node*, guard_t, guard_t>
    find_predecessor_locking(const T& key) const
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

public:
    N_lock_list_set()
    {
        std::unique_ptr<Node> tail{new Node{}};
        std::unique_ptr<Node> head{new Node{}};

        head->next = std::move(tail);

        link_ = std::move(head);
    }

    virtual bool member(const T& key) const override
    {
        auto& pred = find_predecessor(key);

        if (pred.next->is_last()) return false;

        return pred.next->element == key;
    }

    virtual bool remove(const T& key) override
    {
        Node* pred;
        guard_t g1, g2;
        std::tie(pred, g1, g2) = find_predecessor_locking(key);

        if (pred->next->is_last() || pred->next->element != key)
            return false;

        pred->next = std::move(pred->next->next);
        return true;
    }

    virtual bool insert(T key) override
    {
        Node* pred;
        guard_t g1, g2;
        std::tie(pred, g1, g2) = find_predecessor_locking(key);

        if (!pred->next->is_last() && pred->next->element == key)
            return false;

        std::unique_ptr<Node> new_node{new Node{}};
        new_node->element = key;
        new_node->next    = std::move(pred->next);
        pred->next        = std::move(new_node);

        return true;
    }

    virtual std::ostream& format_to(std::ostream& os) const override
    {
        Node* node = &*link_->next;

        if (node->is_last())
            return os << "{}";

        os << "{ " << node->element;

        for (node = &*node->next; !node->is_last(); node = &*node->next) {
            os << ", " << node->element;
        }

        return os << " }";
    }
};
