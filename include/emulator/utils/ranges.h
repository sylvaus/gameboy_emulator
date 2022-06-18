#pragma once

#include <stdexcept>


namespace emulator::utils
{
    template<class T, class F>
    constexpr void for_range(T start, T end, T step, F f)
    {
        if (0 == step)
            throw std::runtime_error("Step shouldn't be 0");
        if (step > 0)
        {
            for (; start < end; start += step)
                f(start);
        }
        else
        {
            for (; start > end; start += step)
                f(start);
        }
    }

    template<class T, class F>
    constexpr void for_range(T start, T end, F f)
    {
        for_range(start, end, static_cast<T>(1), f);
    }

    template<class T, class F>
    constexpr void for_range(T end, F f)
    {
        for_range(static_cast<T>(0), end, static_cast<T>(1), f);
    }
}
