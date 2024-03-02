#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>

#include "include/code/WindowManager.hpp"
#include "include/code/ClAi.hpp"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;

    const QString startWindow = "qrc:/design/windows/MainWindow.qml";

    WindowManager windowManager(&engine, startWindow);
    ClAi sclaiInterface;

    // qmlRegisterType<JsonModel>("JsonModel", 1, 0, "JsonModel");

    engine.rootContext()->setContextProperty("WindowManager", &windowManager);
    engine.rootContext()->setContextProperty("ClAi", &sclaiInterface);

    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
