#include "project.h"

#include "../config/config.h"
#include "../utils/tools.h"
#include "../utils/errors.h"

#include <QDir>
#include <QString>
#include <QFile>
#include <QDebug>
#include <QFileInfo>
#include <QSettings>
#include <QJsonDocument>
#include <QJsonObject>
#include <QMap>


void project::createProject
    (
    const QString& name,
    const QString& profile,
    const QString& dataset
    )
{
    using namespace::std;

    QJsonObject jsonProfiles = tools::getJsonObject(USER_CONFIG_PATH "/profiles.json");

    if (!jsonProfiles.contains(profile))
    {
        throw error::existence::NoSuchProfileError();
    }

    QJsonObject jsonProfile = jsonProfiles[profile].toObject();


    QJsonObject jsonProjects = tools::getJsonObject(USER_CONFIG_PATH "/projects.json");

    if (jsonProjects.contains(name))
    {
        throw error::name::ProjectNameError();
    }

    const QJsonObject& jsonDatasets = tools::getJsonObject(USER_CONFIG_PATH "/datasets.json");

    if (!jsonDatasets.contains(dataset))
    {
        throw error::existence::NoSuchDatasetError();
    }

    const QString& projectPath = DEFAULT_PROJECTS_PATH "/" + name;

    QJsonObject newProject;

    newProject["profile"] = profile;
    newProject["dataset"] = dataset;
    newProject["project_path"] = projectPath;

    const QString& framework = jsonProfile["framework"].toString();
    const QString& scope = jsonProfile["scope"].toString();

    const QJsonObject& jsonFrameworks = tools::getJsonObject(APP_CONFIG_PATH "/frameworks.json");
    const QJsonObject& jsonProject = jsonFrameworks[framework][scope]["project"].toObject();

    QMap<QString, QString> replacements;
    replacements.insert("%{NAME}", name);
    replacements.insert("%{PROJECT_PATH}", projectPath);

    const QString& script = tools::interpretPath(jsonProject["install_script"].toString(), replacements);

    qDebug() << "\033[32m[INFO]: Install Finished with output: " << tools::installProcess(script) << "\033[0m";

    jsonProjects[name] = newProject;

    tools::writeJson(USER_CONFIG_PATH "/projects.json", jsonProjects);
}

void project::deleteProject
    (
    const QString& name,
    bool confirmationDialog
    )
{
    QJsonObject jsonProjects = tools::getJsonObject(USER_CONFIG_PATH "/projects.json");

    if (!jsonProjects.contains(name))
    {
        throw error::existence::NoSuchProjectError();
    }

    tools::deleteFromObject(name, jsonProjects, confirmationDialog);
}

void project::list()
{
    QJsonObject jsonProjects = tools::getJsonObject(USER_CONFIG_PATH "/projects.json");

    qInfo().noquote() << tools::list(jsonProjects).toUtf8();
}
