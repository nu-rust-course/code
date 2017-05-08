#pragma once

#include <atomic>
#include <cassert>
#include <sys/types.h>

#define SYNC  std::memory_order sync = std::memory_order_seq_cst

// Forward declaration:
template<typename T>
class atomic_marked_ptr;

/*
 * This is really a utility class for atomic<marked_ptr<T>> below, which is
 * the main event. The idea is that we want an atomic variable that holds
 * both a pointer and a boolean “mark”. This class is non-atomic version.
 */
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

    static T* unpack_ptr(uintptr_t word) noexcept
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

    T* ptr() const noexcept
    {
        return unpack_ptr(word_);
    }

    bool mark() const noexcept
    {
        return unpack_mark(word_);
    }

    marked_ptr& set_ptr(T* ptr)
    {
        word_ = pack(ptr, mark());
        return *this;
    }

    marked_ptr& set_mark(bool mark)
    {
        word_ = pack(ptr(), mark);
        return *this;
    }

    T& operator*() const noexcept
    {
        return *ptr();
    }

    T* operator->() const noexcept
    {
        return ptr();
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
    friend class atomic_marked_ptr<T>;

    explicit marked_ptr(uintptr_t word) noexcept : word_{word}
    { }
};

template<typename T>
class atomic_marked_ptr
{
    // Representation:
    std::atomic<uintptr_t> base_;

public:
    using contents_t = marked_ptr<T>;

    atomic_marked_ptr() noexcept = default;

    atomic_marked_ptr(const atomic_marked_ptr&) = delete;

    atomic_marked_ptr& operator=(const atomic_marked_ptr&) = delete;

    // Forward any marked_ptr constructor arguments.
    template<typename... Args>
    explicit atomic_marked_ptr(Args... args) noexcept
            : base_{contents_t{std::forward(args)...}.word_}
    { }

    // Assign from a marked_ptr.
    contents_t operator=(contents_t val) noexcept {
        return contents_t{base_ = val.word_};
    }

    bool is_lock_free() const noexcept {
        return base_.is_lock_free();
    }

    contents_t load(SYNC) const noexcept
    {
        return contents_t{base_.load(sync)};
    }

    operator contents_t() const noexcept
    {
        return load();
    }

    T* ptr(SYNC) const noexcept
    {
        return load().ptr();
    }

    T& operator*() const noexcept {
        return *this->ptr();
    }

    T* operator->() const noexcept {
        return this->ptr();
    }

    bool mark(SYNC) const noexcept
    {
        return load().mark();
    }

    void store(contents_t val, SYNC) noexcept
    {
        base_.store(val.word_, sync);
    }

    void set_ptr(T* ptr, SYNC) noexcept
    {
        store({ptr, mark()}, sync);
    }

    void set_mark(bool mark, SYNC) noexcept
    {
        store({ptr(), mark}, sync);
    }

    contents_t exchange(contents_t val, SYNC) noexcept
    {
        return contents_t{base_.exchange(val.word_, sync)};
    }

    bool compare_exchange_weak(contents_t& expected, contents_t val, SYNC) noexcept
    {
        return base_.compare_exchange_weak(expected.word_, val.word_, sync);
    }

    bool compare_exchange_weak(contents_t& expected, contents_t val,
                               std::memory_order success,
                               std::memory_order failure) noexcept
    {
        return base_.compare_exchange_weak(expected.word_, val.word_, success,
                                           failure);
    }

    bool compare_exchange_strong(contents_t& expected, contents_t val, SYNC) noexcept
    {
        return base_.compare_exchange_strong(expected.word_, val.word_, sync);
    }

    bool compare_exchange_strong(contents_t& expected, contents_t val,
                                 std::memory_order success,
                                 std::memory_order failure) noexcept
    {
        return base_.compare_exchange_strong(expected.word_, val.word_, success,
                                             failure);
    }

    bool compare_and_set_weak(T* old_ptr, T* new_ptr,
                              bool old_mark, bool new_mark, SYNC) noexcept
    {
        contents_t expected{old_ptr, old_mark};
        return compare_exchange_weak(expected, {new_ptr, new_mark}, sync);
    }

    bool compare_and_set_weak(T* old_ptr, T* new_ptr,
                              bool old_mark, bool new_mark,
                              std::memory_order success,
                              std::memory_order failure) noexcept
    {
        contents_t expected{old_ptr, old_mark};
        return compare_exchange_weak(expected, {new_ptr, new_mark},
                                     success, failure);
    }

    bool compare_and_set_strong(T* old_ptr, T* new_ptr,
                                bool old_mark, bool new_mark, SYNC) noexcept
    {
        contents_t expected{old_ptr, old_mark};
        return compare_exchange_strong(expected, {new_ptr, new_mark}, sync);
    }

    bool compare_and_set_strong(T* old_ptr, T* new_ptr,
                                bool old_mark, bool new_mark,
                                std::memory_order success,
                                std::memory_order failure) noexcept
    {
        contents_t expected{old_ptr, old_mark};
        return compare_exchange_strong(expected, {new_ptr, new_mark},
                                       success, failure);
    }

};
