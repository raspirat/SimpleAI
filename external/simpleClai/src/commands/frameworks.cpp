#include "frameworks.h"

#include "../config/config.h"
#include "../utils/tools.h"

#include <QCoreApplication>
#include <QDebug>
#include <QJsonObject>

void frameworks::list()
{
    QJsonObject jsonFrameworks = tools::getJsonObject(APP_CONFIG_PATH "/frameworks.json");

    qInfo().noquote() << tools::list(jsonFrameworks).toUtf8();
}
