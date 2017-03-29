#pragma once

#include "Set_base.h"

#include "marked_ptr.h"

#include <iostream>
#include <memory>
#include <mutex>
#include <tuple>

#undef Set
#define Set Lock_free_list_set

template <typename T>
class Lock_free_list_set : public Set_base<T>
{
protected:
    ///
    /// TYPES
    ///

    struct Node;
    using link_t  = std::atomic<marked_ptr<Node>>;

    struct Node : Node_base<T> {
        T      element;
        link_t link;

        const T& get_element() const {
            return element;
        }

        const Node* get_next() const {
            return link.ptr();
        }

        Node* get_next() {
            return link.ptr();
        }

        bool get_mark() const {
            return link.mark();
        }
    };

    ///
    /// VARIABLES
    ///

    // Head of the list
    link_t link_;

    ///
    /// FUNCTIONS
    ///

    virtual Node& find_predecessor_deleting(const T& key)
    {
        retry:

        Node* prev = link_.ptr();
        Node* curr = prev->link.ptr();

        for (;;) {
            while (curr->get_mark()) {
                if (! prev->link.compare_and_set_weak(curr, curr->get_next(),
                                                      false, false))
                    goto retry;

                delete curr;
                curr = curr->get_next();
            }

            if (curr->element >= key || curr->is_last()) return *prev;

            prev = curr;
            curr = curr->get_next();
        }
    }

    bool matches(const Node& curr, const T& key) const
    {
        return !curr.is_last() && curr.element == key;
    }

    bool matches_unmarked(const Node& curr, const T& key) const
    {
        return matches(curr, key) && !curr.get_mark();
    }

public:
    Lock_free_list_set()
    {
        marked_ptr<Node> tail{new Node{}, false};
        marked_ptr<Node> head{new Node{}, false};

        head->link = tail;
        link_      = head;
    }

    ~Lock_free_list_set()
    {
        auto curr = link_.ptr();

        while (curr != nullptr) {
            auto next = curr->get_next();
            delete curr;
            curr = next;
        }
    }

    virtual bool member(const T& key) const override
    {
        for (auto curr = link_->get_next();
             !curr->is_last();
             curr = curr->get_next())
            if (key <= curr->element) return matches_unmarked(*curr, key);

        return false;
    }

    virtual bool remove(const T& key) override
    {
        retry:

        Node& prev = find_predecessor_deleting(key);
        Node& curr = *prev.link;

        if (!matches(curr, key)) return false;

        Node& next = *curr.link;

        if (! curr.link.compare_and_set_weak(&next, &next, false, true))
            goto retry;

        if (prev.link.compare_and_set_strong(&curr, &next, false, false))
            delete &curr;

        return true;
    }

    virtual bool insert(T key) override
    {
        Node* node = new Node;
        node->element = std::move(key);
        node->link.mark(false);

        retry:

        Node& prev = find_predecessor_deleting(key);
        Node& curr = *prev.link;

        if (matches(curr, key)) return false;

        node->link.ptr(&curr);

        if (! prev.link.compare_and_set_weak(&curr, node, false, false))
            goto retry;

        return true;
    }

    virtual const Node* head() const override
    {
        return &*link_;
    }
};
