#pragma once

#include "List_set.h"

#include <mutex>

#undef Set
#define Set One_lock_list_set

template <typename T>
class One_lock_list_set : public List_set<T>
{
private:
    using super   = List_set<T>;
    using guard_t = std::lock_guard<std::mutex>;

    mutable std::mutex lock_;

public:
    virtual bool member(const T& key) const override
    {
        guard_t guard{lock_};
        return super::member(key);
    }

    virtual bool remove(const T& key) override
    {
        guard_t guard{lock_};
        return super::remove(key);
    }

    virtual bool insert(T key) override
    {
        guard_t guard{lock_};
        return super::insert(key);
    }

    virtual std::ostream& format_to(std::ostream& os)
    {
        guard_t guard{lock_};
        return super::format_to(os);
    }
};
