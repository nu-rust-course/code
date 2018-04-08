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
    using link_t  = atomic_marked_ptr<Node>;

    struct Node : Node_base<T> {
        T      element;
        link_t link;

        virtual const T& get_element() const final override {
            return element;
        }

        virtual const Node* get_next() const final override {
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

        for (;;) {
            Node* curr = prev->get_next();

            while (curr->get_mark()) {
                if (! prev->link.compare_and_set_weak(curr, curr->get_next(),
                                                      false, false))
                    goto retry;

                Node* next = curr->get_next();
                // This leaks memory. See explanation below.
                // delete curr;
                curr = next;
            }

            if (curr->element >= key || curr->is_last()) return *prev;

            prev = curr;
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
        link_ = marked_ptr<Node>{new Node{}, false};
        link_->link = marked_ptr<Node>{new Node{}, false};
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
        Node* curr = link_->get_next();

        while (!curr->is_last() && key > curr->element) {
            curr = curr->get_next();
        }

        return matches_unmarked(*curr, key);
    }

    virtual bool remove(const T& key) override
    {
        for (;;) {
            Node& prev = find_predecessor_deleting(key);
            Node& curr = *prev.link;

            if (!matches(curr, key)) return false;

            Node& next = *curr.link;

            if (curr.link.compare_and_set_weak(&next, &next, false, true)) {
                if (prev.link.compare_and_set_strong(
                        &curr, &next, false, false))
                    // Want to delete here, but can't do it safely because
                    // another thread might still be traversing this node.
                    // delete &curr;

                return true;
            }
        }
    }

    virtual bool insert(T key) override
    {
        Node* node = new Node;
        node->element = std::move(key);
        node->link.set_mark(false);

        for (;;) {
            Node& prev = find_predecessor_deleting(key);
            Node& curr = *prev.link;

            if (matches(curr, key)) {
                delete node;
                return false;
            }

            node->link.set_ptr(&curr);

            if (prev.link.compare_and_set_weak(&curr, node, false, false))
                return true;
        }
    }

protected:
    virtual const Node* head() const override
    {
        return &*link_;
    }
};
