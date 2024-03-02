#include <QCoreApplication>
#include <QDebug>

#include "src/config/config.h" // edit all default paths and vars in config.h
#include "src/core/parser.h"

int main(int argc, char *argv[])
{
    QCoreApplication a(argc, argv);

    QCoreApplication::setApplicationName(APP_NAME);
    QCoreApplication::setApplicationVersion(APP_VERSION);

    clparser::parseArgs(argc, argv);

    // qInfo() << "\033[32m[INFO]: Finished!\033[0m";

    return 0;
}
