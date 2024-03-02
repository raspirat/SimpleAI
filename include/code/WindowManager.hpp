#ifndef WINDOWMANAGER_H
#define WINDOWMANAGER_H

#include <QObject>
#include <QQmlApplicationEngine>
#include <QQuickWindow>


class WindowManager : public QObject
{

Q_OBJECT

private:
    QQmlApplicationEngine * engine_;
    QQuickWindow * currentWindow_ {};

public:
    explicit WindowManager(QQmlApplicationEngine * engine, const QString & startWindowPath, QObject * parent = nullptr);

public slots:
    void openWindow(const QString & windowPath);
    void switchWindows(const QString & windowPath);
    void openWindowAlongside(const QString & windowPath);
    void closeWindow();
};

#endif // WINDOWMANAGER_H
