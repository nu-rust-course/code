#pragma once

#include "Set_base.h"

#include <iostream>
#include <memory>

#undef Set
#define Set List_set

template <typename T>
class List_set : public Set_base<T>
{
protected:
    struct Node;
    using link_t = std::unique_ptr<Node>;

    struct Node {
        T element;
        link_t next;

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
        Node* ptr;

        for (ptr = &*link_; !ptr->next->is_last(); ptr = &*ptr->next)
            if (key <= ptr->next->element) break;

        return *ptr;
    }

public:
    List_set()
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
        auto& pred = find_predecessor(key);
        if (pred.next->is_last() || pred.next->element != key) return false;

        pred.next = std::move(pred.next->next);
        return true;
    }

    virtual bool insert(T key) override
    {
        auto& pred = find_predecessor(key);
        if (!pred.next->is_last() && pred.next->element == key) return false;

        std::unique_ptr<Node> new_node{new Node{}};
        new_node->element = key;
        new_node->next    = std::move(pred.next);
        pred.next         = std::move(new_node);

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
