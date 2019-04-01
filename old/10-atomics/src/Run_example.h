#pragma once

#include <iostream>
#include <memory>
#include <thread>

template<typename Example>
class Run_example
{
    std::unique_ptr<Example> example_;

public:
    Run_example() : example_{nullptr}
    {
        while (example_ == nullptr)
            attempt();
    }

    Run_example(size_t n) : example_{nullptr}
    {
        for (size_t i = 0; i < n && example_ == nullptr; ++i)
            attempt();
    }

    std::ostream& fmt(std::ostream& os) const
    {
        if (example_ != nullptr) {
            os << "Found{ ";
            example_->fmt(os);
            return os << " }";
        } else {
            return os << "Not_found{}";
        }
    }

private:
    void attempt()
    {
        auto example = std::make_unique<Example>();

        std::thread tl([&]() { example->left(); });
        std::thread tr([&]() { example->right(); });

        tl.join();
        tr.join();

        if (!example->is_valid()) example_ = std::move(example);
    }
};

template <typename T>
std::ostream& operator<<(std::ostream& os, const Run_example<T>& runner)
{
    return runner.fmt(os);
}

