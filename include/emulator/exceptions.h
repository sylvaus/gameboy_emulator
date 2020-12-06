#pragma once

#include <exception>
#include <string>

namespace emulator
{
    class BaseEmulatorException : public std::exception
    {
    public:
        BaseEmulatorException(): BaseEmulatorException("Error happened") {};
        explicit BaseEmulatorException(std::string what): what_(std::move(what)) {};

        [[nodiscard]] const char * what () const noexcept override {
            return what_.c_str();
        }
    private:
        const std::string what_;
    };

    class LoadCartridgeError: public BaseEmulatorException
    {
    public:
        using BaseEmulatorException::BaseEmulatorException;
    };

}
