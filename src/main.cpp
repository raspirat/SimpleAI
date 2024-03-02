#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>

#include "include/code/WindowManager.hpp"
#include "include/code/ClAi.hpp"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;

    qDebug() << "123";

    const QString startWindow = "qrc:/design/windows/MainWindow.qml";

    qDebug() << startWindow;

    WindowManager windowManager(&engine, startWindow);
    ClAi sclaiInterface;

    // qmlRegisterType<JsonModel>("JsonModel", 1, 0, "JsonModel");

    qDebug() <<  "42";

    engine.rootContext()->setContextProperty("WindowManager", &windowManager);
    engine.rootContext()->setContextProperty("ClAi", &sclaiInterface);

    qDebug() << "engine";

    if (engine.rootObjects().isEmpty())
        return -1;

    qDebug() <<  "return";

    return app.exec();
}
