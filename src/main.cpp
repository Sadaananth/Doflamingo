#include "src/Logger.hpp"

using namespace Sada;

int main()
{
    Logger::instance().add_sink(Logger::Sink::console, LogLevel::Info);
    LOG_INFO << "Creating Application";

    return 0;
}
