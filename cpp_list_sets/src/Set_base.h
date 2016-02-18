#pragma once

#include <iostream>

template<typename T>
class Set_base
{
public:
    virtual bool insert(T)              =0;
    virtual bool member(const T&) const =0;
    virtual bool remove(const T&)       =0;

    virtual std::ostream& format_to(std::ostream&) const =0;
};

template <typename T>
std::ostream& operator<<(std::ostream& os, const Set_base<T>& set)
{
    return set.format_to(os);
}
