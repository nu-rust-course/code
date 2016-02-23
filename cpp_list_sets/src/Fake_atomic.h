#pragma once

template<typename T>
class Fake_atomic
{
    T data_;

public:
    T load() const
    {
        return data_;
    }

    void store(T desired)
    {
        data_ = desired;
    }



    T exchange(T desired) // or swap
    {
        // ATOMIC {
        T result = data_;
        data_ = desired;
        return result;
        // } END ATOMIC
    }



    bool compare_and_set(T expected, T desired)
    {
        // ATOMIC {
        if (data_ == expected) {
            data_ = desired;
            return true;
        } else {
            return false;
        }
        // } END ATOMIC
    }



    bool compare_exchange(T& expected, T desired)
    {
        // ATOMIC {
        if (data_ == expected) {
            data_ = desired;
            return true;
        } else {
            expected = data_;
            return false;
        }
        // } END ATOMIC
    }

};
