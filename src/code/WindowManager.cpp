#include "include/code/WindowManager.hpp"

#include <QQmlComponent>
#include <QQuickWindow>

#include <algorithm>

WindowManager::WindowManager(QQmlApplicationEngine * engine, const QString & startWindowPath, QObject * parent) : QObject(parent)
{
    this->engine_ = engine;
    this->openWindow(startWindowPath);
}


void WindowManager::openWindow(const QString & windowPath)
{
    if (this->currentWindow_)
        return;

    this->engine_->load(QUrl(windowPath));

    QList<QObject *> rootObjects = this->engine_->rootObjects();

    QObject * rootObject = rootObjects.last();

    this->currentWindow_ = qobject_cast<QQuickWindow*>(rootObject);

    if (this->currentWindow_)
        this->currentWindow_->show();
}

void WindowManager::closeWindow()
{
    if (this->currentWindow_)
    {
        this->currentWindow_->close();
        this->currentWindow_ = nullptr;
    }
}

void WindowManager::switchWindows(const QString & windowPath)
{
    this->closeWindow();
    this->openWindow(windowPath);
}

void WindowManager::openWindowAlongside(const QString & windowPath) { }
