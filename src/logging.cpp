#include <iostream>

#include "emulator/logging.h"

namespace emulator
{
    LogManager::LogManager(): logging_level_{INFO_LEVEL}
    {}

    LogManager::~LogManager()
    {
        flush();
    }

    void LogManager::set_logging_level(uint8_t level, const std::string &)
    {
        logging_level_ = level;
    }

    void LogManager::log(uint8_t level, const std::string& name, const std::string& text) const
    {
        if (level < logging_level_)
            return;

        const auto& level_name = LOGGING_LEVEL_NAME_MAP.get(level);
        std::cout << level_name << ':' << name << ':' << text << '\n';
    }

    void LogManager::flush()
    {
        std::cout.flush();
    }

    void Logger::log(uint8_t level, const std::string& text)
    {
        manager_->log(level, name_, text);
    }

    void Logger::trace(const std::string& text)
    {
        log(TRACE_LEVEL, text);
    }

    void Logger::debug(const std::string& text)
    {
        log(DEBUG_LEVEL, text);
    }

    void Logger::info(const std::string& text)
    {
        log(INFO_LEVEL, text);
    }

    void Logger::warn(const std::string& text)
    {
        log(WARN_LEVEL, text);
    }

    void Logger::error(const std::string& text)
    {
        log(ERROR_LEVEL, text);
    }

    void Logger::flush()
    {
        manager_->flush();
    }

    Logger::Logger(std::shared_ptr<LogManager> manager, std::string name):
        manager_(std::move(manager)), name_(std::move(name))
    {}

    Logger Logging::get_logger(const std::string& name)
    {
        return Logger(get_manager(), name);
    }

    void Logging::set_logging_level(uint8_t level)
    {
        get_manager()->set_logging_level(level, "");
    }

    std::shared_ptr<LogManager> Logging::get_manager()
    {
        static auto manager = std::make_shared<LogManager>();
        return manager;
    }
}