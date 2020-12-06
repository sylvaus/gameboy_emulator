#pragma once

#include <unordered_map>
#include <optional>

namespace emulator::utils
{
    template<class K, class V>
    class UnMutableDefaultMap
    {
    public:
        explicit UnMutableDefaultMap(const V default_val, std::unordered_map<K, V> map) noexcept:
            default_(std::move(default_val)), map_(std::move(map))
        {}

        const V& get(const K &key) const noexcept
        {
            if (auto value = map_.find(key); value != map_.end())
                return value->second;
            return default_;
        }

    private:
        const V default_;
        const std::unordered_map<K, V> map_;
    };
}