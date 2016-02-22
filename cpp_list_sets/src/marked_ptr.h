#pragma once

#include <atomic>
#include <cassert>
#include <sys/types.h>

template<typename T>
class marked_ptr
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
    marked_ptr() noexcept = default;

    marked_ptr(std::nullptr_t) noexcept
            : marked_ptr{nullptr, false}
    { }

    marked_ptr(T* ptr, bool mark) noexcept
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

    marked_ptr& pointer(T* pointer)
    {
        word_ = pack(pointer, mark());
        return *this;
    }

    marked_ptr& mark(bool mark)
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

    bool operator==(const marked_ptr& other)
    {
        return word_ == other.word_;
    }

    bool operator!=(const marked_ptr& other)
    {
        return word_ != other.word_;
    }

private:
    friend class std::atomic<marked_ptr<T>>;

    explicit marked_ptr(uintptr_t word) noexcept : word_{word}
    { }
};

template<typename T>
class std::atomic<marked_ptr<T>>
{
    // Representation:
    atomic<uintptr_t> base_;

public:
    using contents_t       = marked_ptr<T>;

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

    operator bool() const noexcept
    { return bool{load()}; }

    T& operator*() noexcept
    { return *load(); }

    const T& operator*() const noexcept
    { return *load(); }

    T* operator->() noexcept
    { return load().operator->(); }

    const T* operator->() const noexcept
    { return load().operator->(); }

    bool is_lock_free() const noexcept
    { return base_.is_lock_free(); }

    void store(contents_t val, std::memory_order sync = std::memory_order_seq_cst) noexcept
    { base_.store(val.word_, sync); }

    void store(contents_t val, std::memory_order sync = std::memory_order_seq_cst) volatile noexcept
    { base_.store(val.word_, sync); }

    contents_t load(std::memory_order sync = std::memory_order_seq_cst) noexcept
    { return contents_t{base_.load(sync)}; }

    contents_t load(std::memory_order sync = std::memory_order_seq_cst) volatile noexcept
    { return contents_t{base_.load(sync)}; }

    const contents_t load(std::memory_order sync = std::memory_order_seq_cst) const noexcept
    { return contents_t{base_.load(sync)}; }

    const contents_t load(std::memory_order sync = std::memory_order_seq_cst) const volatile noexcept
    { return contents_t{base_.load(sync)}; }

    operator contents_t() noexcept
    { return load(); }

    operator contents_t() volatile noexcept
    { return load(); }

    operator const contents_t() const noexcept
    { return load(); }

    operator const contents_t() const volatile noexcept
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

