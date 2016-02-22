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
    struct Node;
    using link_t  = std::atomic<marked_ptr<Node>>;

    struct Node : Node_base<T> {
        T      element;
        link_t link;

        const T& get_element() const { return element; }
        const Node* get_next() const { return &*link; }
    };

    // Head of the list
    link_t link_;

    // Finds the predecessor node of the first node whose element is not less
    // than `key`. That is, if `key` is in the list then it will be found in
    // the result's successor node, and if `key` is not in the list then it
    // belongs between the result and its successor.
    virtual Node& find_predecessor_deleting(const T& key)
    {
        retry:
        for (;;) {
            Node* prev = link_.load().pointer();
            Node* curr = prev->link.load().pointer();

            for (;;) {
                marked_ptr<Node> marked = curr->link.load();
                Node* succ = marked.pointer();

                while (marked.mark()) {
                    marked_ptr<Node> expected{curr, false};
                    if (!prev->link.compare_exchange_weak(expected,
                                                          {succ, false}))
                        goto retry;

                    delete curr;

                    curr = succ;
                    marked = curr->link.load();
                    succ = marked.pointer();
                }

                if (curr->element >= key || curr->is_last()) return *prev;

                prev = curr;
                curr = succ;
            }
        }
    }

    bool matches(const Node& curr, const T& key) const
    {
        return !curr.is_last() && curr.element == key;
    }

    bool matches_unmarked(const Node& curr, const T& key) const
    {
        return matches(curr, key) && !curr.link.load().mark();
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
        Node* curr = &*link_;

        while (curr != nullptr) {
            Node* next = &*curr->link;
            delete curr;
            curr = next;
        }
    }

    virtual bool member(const T& key) const override
    {
        for (auto curr = &*link_->link; !curr->is_last(); curr = &*curr->link)
            if (key <= curr->element) return matches_unmarked(*curr, key);

        return false;
    }

    virtual bool remove(const T& key) override
    {
        for (;;) {
            Node& prev = find_predecessor_deleting(key);
            Node& curr = *prev.link;

            if (!matches(curr, key)) return false;

            Node& next = *curr.link;

            marked_ptr<Node> expected{&next, false};

            if (curr.link.compare_exchange_weak(expected, {&next, true})) {
                expected = {&curr, false};

                if (prev.link.compare_exchange_strong(expected, {&next, false}))
                    delete &curr;

                return true;
            }
        }
    }

    virtual bool insert(T key) override
    {
        Node* node = new Node;

        for (;;) {
            Node& prev = find_predecessor_deleting(key);
            Node& curr = *prev.link;

            if (matches(curr, key)) return false;

            marked_ptr<Node> expected{&curr, false};
            node->element = key;
            node->link    = expected;

            if (prev.link.compare_exchange_weak(expected, {node, false})) {
                return true;
            }
        }
    }

    virtual const Node* head() const override
    {
        return &*link_;
    }
};
