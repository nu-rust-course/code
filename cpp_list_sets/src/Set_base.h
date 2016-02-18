#pragma once

#include "Node_base.h"

#include <iostream>

template<typename T>
class Set_base
{
public:
    virtual bool insert(T)              =0;
    virtual bool member(const T&) const =0;
    virtual bool remove(const T&)       =0;

    virtual const Node_base<T>* head() const =0;

    virtual std::ostream& format_to(std::ostream& os) const
    {
        const Node_base<T>* node = head()->get_next();

        if (node->is_last())
            return os << "{}";

        os << "{ " << node->get_element();

        for (node = node->get_next(); !node->is_last(); node = node->get_next()) {
            os << ", " << node->get_element();
        }

        return os << " }";
    }
};

template <typename T>
std::ostream& operator<<(std::ostream& os, const Set_base<T>& set)
{
    return set.format_to(os);
}
