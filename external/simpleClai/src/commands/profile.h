#ifndef PROFILE_H
#define PROFILE_H

#include <iostream>

#include <QString>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>

namespace profile
{
int createProfile
    (
    const QString& name,
    const QString& framework,
    const QString& scope
    );

int deleteProfile
    (
    const QString& name,
    bool confirmationDialog = true
    );

void list();
}

#endif // PROFILE_H
