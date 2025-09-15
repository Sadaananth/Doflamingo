#include "app-window.h"

int main(int argc, char **argv)
{
    auto app_window = AppWindow::create();
    app_window->run();
    return 0;
}
