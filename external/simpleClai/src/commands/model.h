#ifndef MODEL_H
#define MODEL_H

#include <iostream>

#include <QString>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>

namespace model
{
int createModel
    (
    const QString& name,
    const QString& project,
    const QString& model
    );

int trainModel
    (
    const QString& name,
    const QString& project
    );

int deleteModel
    (
    const QString& name,
    const QString& project,
    bool confirmationDialog = true
    );

void list
    (
    const QString& framework,
    const QString& scope
    );
}

#endif // MODEL_H
