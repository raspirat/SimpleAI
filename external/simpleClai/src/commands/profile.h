#ifndef PROFILE_H
#define PROFILE_H

#include <iostream>

#include <QString>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>

namespace profile
{
void createProfile
    (
    const QString& name,
    const QString& framework,
    const QString& scope
    );

void deleteProfile
    (
    const QString& name,
    bool confirmationDialog = true
    );

void list();
}

#endif // PROFILE_H
