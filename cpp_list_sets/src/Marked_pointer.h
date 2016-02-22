#pragma once

#include <atomic>
#include <cassert>
#include <sys/types.h>

template<typename T>
class Marked_pointer
{
    // Representation:
    uintptr_t word_;

    static constexpr uintptr_t MARK_MASK = 1;
    static constexpr uintptr_t PTR_MASK  = ~MARK_MASK;

    static uintptr_t pack(T* ptr, bool mark) noexcept
    {
        uintptr_t word = reinterpret_cast<uintptr_t>(ptr);
        assert((PTR_MASK & word) == word);
        return (PTR_MASK & word) | (MARK_MASK & mark);
    }

    static T* unpack_pointer(uintptr_t word) noexcept
    {
        return reinterpret_cast<T*>(word & PTR_MASK);
    }

    static bool unpack_mark(uintptr_t word) noexcept
    {
        return (word & MARK_MASK) == 1;
    }

public:
    Marked_pointer() noexcept = default;

    Marked_pointer(std::nullptr_t) noexcept
            : Marked_pointer{nullptr, false}
    { }

    Marked_pointer(T* ptr, bool mark) noexcept
            : word_{pack(ptr, mark)}
    { }

    T* pointer() const noexcept
    {
        return unpack_pointer(word_);
    }

    bool mark() const noexcept
    {
        return unpack_mark(word_);
    }

    Marked_pointer& pointer(T* pointer)
    {
        word_ = pack(pointer, mark());
        return *this;
    }

    Marked_pointer& mark(bool mark)
    {
        word_ = pack(pointer(), mark);
        return *this;
    }

    operator bool() const noexcept
    {
        return mark();
    }

    T& operator*() const noexcept
    {
        return *pointer();
    }

    T* operator->() const noexcept
    {
        return pointer();
    }

    bool operator==(const Marked_pointer& other)
    {
        return word_ == other.word_;
    }

    bool operator!=(const Marked_pointer& other)
    {
        return word_ != other.word_;
    }

private:
    friend class std::atomic<Marked_pointer<T>>;

    explicit Marked_pointer(uintptr_t word) noexcept : word_{word}
    { }
};

template<typename T>
class std::atomic<Marked_pointer<T>>
{
    // Representation:
    atomic<uintptr_t> base_;

public:
    using contents_t = Marked_pointer<T>;

    atomic() noexcept = default;

    atomic(const atomic&) = delete;

    atomic& operator=(const atomic&) = delete;

    atomic& operator=(const atomic&) volatile = delete;

    template <typename... Args>
    atomic(Args... args) noexcept
        : base_{contents_t{args...}.word_}
    { }

    contents_t operator=(contents_t val) noexcept
    { return contents_t{base_ = val.word_}; }

    contents_t operator=(contents_t val) volatile noexcept
    { return contents_t{base_ = val.word_}; }

    operator bool() noexcept
    { return bool{load()}; }

    T& operator*() noexcept
    { return *load(); }

    T* operator->() noexcept
    { return load().operator->(); }

    bool is_lock_free() noexcept
    { return base_.is_lock_free(); }

    void store(contents_t val, std::memory_order sync = std::memory_order_seq_cst) noexcept
    { base_.store(val.word_, sync); }

    void store(contents_t val, std::memory_order sync = std::memory_order_seq_cst) volatile noexcept
    { base_.store(val.word_, sync); }

    contents_t load(std::memory_order sync = std::memory_order_seq_cst) noexcept
    { return contents_t{base_.load(sync)}; }

    contents_t load(std::memory_order sync = std::memory_order_seq_cst) volatile noexcept
    { return contents_t{base_.load(sync)}; }

    operator contents_t() const noexcept
    { return load(); }

    operator contents_t() const volatile noexcept
    { return load(); }

    contents_t exchange(contents_t val, std::memory_order sync = std::memory_order_seq_cst) noexcept
    { return contents_t{base_.exchange(val.word_, sync)}; }

    contents_t exchange(contents_t val, std::memory_order sync = std::memory_order_seq_cst) volatile noexcept
    { return contents_t{base_.exchange(val.word_, sync)}; }

    bool compare_exchange_weak(contents_t& expected, contents_t val,
                               std::memory_order sync = std::memory_order_seq_cst) noexcept
    { return base_.compare_exchange_weak(expected.word_, val.word_, sync); }

    bool compare_exchange_weak(contents_t& expected, contents_t val,
                               std::memory_order sync = std::memory_order_seq_cst) volatile noexcept
    { return base_.compare_exchange_weak(expected.word_, val.word_, sync); }

    bool compare_exchange_weak(contents_t& expected, contents_t val,
                               std::memory_order success, std::memory_order failure) noexcept
    { return base_.compare_exchange_weak(expected.word_, val.word_, success, failure); }

    bool compare_exchange_weak(contents_t& expected, contents_t val,
                               std::memory_order success, std::memory_order failure) volatile noexcept
    { return base_.compare_exchange_weak(expected.word_, val.word_, success, failure); }

    bool compare_exchange_strong(contents_t& expected, contents_t val,
                                 std::memory_order sync = std::memory_order_seq_cst) noexcept
    { return base_.compare_exchange_strong(expected.word_, val.word_, sync); }

    bool compare_exchange_strong(contents_t& expected, contents_t val,
                                 std::memory_order sync = std::memory_order_seq_cst) volatile noexcept
    { return base_.compare_exchange_strong(expected.word_, val.word_, sync); }

    bool compare_exchange_strong(contents_t& expected, contents_t val,
                                 std::memory_order success, std::memory_order failure) noexcept
    { return base_.compare_exchange_strong(expected.word_, val.word_, success, failure); }

    bool compare_exchange_strong(contents_t& expected, contents_t val,
                                 std::memory_order success, std::memory_order failure) volatile noexcept
    { return base_.compare_exchange_strong(expected.word_, val.word_, success, failure); }
};

