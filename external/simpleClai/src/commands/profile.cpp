#include "profile.h"

#include "../config/config.h"
#include "../utils/tools.h"
#include "../utils/errors.h"

#include <iostream>
#include <cstdint>
#include <exception>

#include <QCoreApplication>
#include <QString>
#include <QFile>
#include <QDir>
#include <QDebug>
#include <QJsonDocument>
#include <QJsonObject>
#include <QProcessEnvironment>


void profile::createProfile
    (
    const QString& name,
    const QString& framework,
    const QString& scope
    )
{
    using namespace::std;

    QJsonObject jsonProfiles = tools::getJsonObject(USER_CONFIG_PATH "/profiles.json");

    if (jsonProfiles.contains(name))
    {
        throw error::name::ProfileNameError();
    }

    qDebug() << "\033[90m[DEBUG]: Framework is:" << framework << "\033[0m";

    const QJsonObject& jsonFrameworks = tools::getJsonObject(APP_CONFIG_PATH "/frameworks.json");

    if (!jsonFrameworks.contains(framework))
    {
        throw error::existence::NoSuchFrameworkError();
    }

    qDebug() << "\033[90m[DEBUG]: Scope is:" << scope << "\033[0m";

    const QJsonObject& jsonScopes = jsonFrameworks[framework].toObject();

    qDebug() << "\033[90m[DEBUG]: Json-scopes is:" << jsonScopes << "\033[0m";

    if (!jsonScopes.contains(scope))
    {
        throw error::existence::NoSuchScopeError();
    }

    const QJsonObject& jsonScope = jsonScopes[scope].toObject();

    QProcessEnvironment env = QProcessEnvironment::systemEnvironment();

    QString profilePath = env.value("PROFILES_PATH");

    if (profilePath.isEmpty())
    {
        error::environment::PROFILES_PATH_Error error;

        profilePath = DEFAULT_PROFILES_PATH;

        qInfo() << error.what() << "\033[36m"
                << "Default:" << profilePath
                << "\033[0m";

        // set the $SA_PROFILE_PATH for debian
        qDebug() << "\033[90m[DEBUG]: Script executed with output:"
                 << tools::installProcess(APP_SCRIPTS_PATH "/set_debian_env.sh PROFILES_PATH " + profilePath)
                 << "\033[0m";
    }

    const QString& thisProfilePath = profilePath + "/" + name;

    qInfo() << "\033[32m[INFO]: Your profile will be stored in:\033[35m" << thisProfilePath << "\033[0m";

    QMap<QString, QString> replacements;
    replacements.insert("%{NAME}", name);
    replacements.insert("%{PROFILE_PATH}", thisProfilePath);

    const QString& pythonPath = tools::interpretPath(jsonScopes["python_path"].toString(), replacements);

    QJsonObject newProfile;

    newProfile["scope"] = scope;
    newProfile["framework"] = framework;
    newProfile["profile_path"] = thisProfilePath;
    newProfile["python_path"] = pythonPath;

    const QJsonObject& jsonProfile = jsonScope["profile"].toObject();

    const QString& script = tools::interpretPath(jsonProfile["install_script"].toString(), replacements);

    qDebug() << "\033[32m[INFO]: Install Finished with output: " << tools::installProcess(script, QStringList() << QString("PROFILE_PATH=") + profilePath) << "\033[0m";

    jsonProfiles[name] = newProfile;

    tools::writeJson(USER_CONFIG_PATH "/profiles.json", jsonProfiles);
}

void profile::deleteProfile
    (
    const QString& name,
    bool confirmationDialog
    )
{
    QJsonObject jsonProfiles = tools::getJsonObject(USER_CONFIG_PATH "/profiles.json");

    if (!jsonProfiles.contains(name))
    {
        throw error::existence::NoSuchProfileError();
    }

    tools::deleteFromObject(name, jsonProfiles, confirmationDialog);
}

void profile::list()
{
    const QJsonObject& jsonProfiles = tools::getJsonObject(USER_CONFIG_PATH "/profiles.json");

    qInfo().noquote() << tools::list(jsonProfiles).toUtf8();
}


