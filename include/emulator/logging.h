#pragma once

#include <cstdint>
#include <string>
#include <memory>

#include "emulator/utils/map.h"

namespace emulator
{
    constexpr uint8_t TRACE_LEVEL = 10;
    constexpr uint8_t DEBUG_LEVEL = 20;
    constexpr uint8_t INFO_LEVEL  = 30;
    constexpr uint8_t WARN_LEVEL  = 40;
    constexpr uint8_t ERROR_LEVEL = 50;

    const utils::UnMutableDefaultMap<uint8_t, std::string> LOGGING_LEVEL_NAME_MAP
    { "Custom", {
        {TRACE_LEVEL, "Trace"},
        {DEBUG_LEVEL, "Debug"},
        {INFO_LEVEL, "Info"},
        {WARN_LEVEL, "Warn"},
        {ERROR_LEVEL, "Error"},
    }};

    class LogManager
    {
    public:
        LogManager();
        ~LogManager();
        void set_logging_level(uint8_t level, const std::string &name);
        void log(uint8_t level, const std::string& name, const std::string& text) const;
        static void flush();
    private:
        uint8_t logging_level_;
    };

    class Logging;

    class Logger
    {
    friend class Logging;
    public:
        void log(uint8_t level, const std::string& text);
        void trace(const std::string& text);
        void debug(const std::string& text);
        void info(const std::string& text);
        void warn(const std::string& text);
        void error(const std::string& text);

        void flush();
    private:
        LogManager& manager_;
        std::string name_;

        Logger(LogManager& manager, std::string name);
    };

    class Logging
    {
    public:
        static Logger& get_logger(const std::string& name = "");
        static void set_logging_level(uint8_t level);
    private:
        static LogManager& get_manager();
    };
}


