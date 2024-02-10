#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QIcon>
// #include <engine.h>

int main(int argc, char *argv[])
{
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
#endif
    QGuiApplication app(argc, argv);
    app.setWindowIcon(QIcon("assets/logo.ico"));

    QQmlApplicationEngine appEngine;

    qmlRegisterSingletonType(QStringLiteral("qrc:/components/Style.qml"),"Style",1,0,"Style");
    qmlRegisterSingletonType(QStringLiteral("qrc:/Font.qml"),"Font",1,0,"Font");

    const QUrl url(QStringLiteral("qrc:/main.qml"));
    QObject::connect(&appEngine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);
    appEngine.load(url);

    return app.exec();
}
