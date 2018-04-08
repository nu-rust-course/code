#pragma once

#include <array>
#include <atomic>

class fifo_full {};
class fifo_empty {};

template <typename Element, int capacity>
class Wf_SRSW_FIFO
{
public:
    void enq(Element);
    Element deq();

private:
    std::array<Element, capacity> data_;
    std::atomic<unsigned long> head_{0}, tail_{0};
};

template <typename Element, int capacity>
void Wf_SRSW_FIFO<Element, capacity>::enq(Element x)
{
    if (tail_ - head_ == capacity) throw fifo_full();

    data_[tail_ % capacity] = x;
    ++tail_;
}

template <typename Element, int capacity>
Element Wf_SRSW_FIFO<Element, capacity>::deq()
{
    if (tail_ == head_) throw fifo_empty();

    Element result = data_[head_ % capacity];
    ++head_;
    return result;
}
