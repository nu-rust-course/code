#pragma once

#include <memory>

template<typename T>
struct Node_base
{
    virtual const T& get_element() const        =0;
    virtual const Node_base* get_next() const   =0;

    bool is_last() const
    {
        return get_next() == nullptr;
    }
};
