// #include <Python.h>

#include "model.h"

#include "../config/config.h"
#include "../utils/tools.h"
#include "../utils/errors.h"


#include <QProcess>
#include <QString>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>
#include <QDir>

void model::createModel
    (
    const QString& name,
    const QString& project,
    const QString& model
    )
{
    using namespace::std;

    QJsonObject jsonProjects = tools::getJsonObject(USER_CONFIG_PATH "/projects.json");

    if (!jsonProjects.contains(project))
    {
        throw error::existence::NoSuchProjectError();
    }

    QJsonObject jsonProject = jsonProjects[project].toObject();
    QJsonObject jsonUserModels = jsonProject["models"].toObject();

    if (jsonUserModels.contains(name) && ! jsonUserModels.isEmpty())
        throw error::name::ModelNameError();


    const QString& profile = jsonProject["profile"].toString();
    const QJsonObject& jsonProfiles = tools::getJsonObject(USER_CONFIG_PATH "/profiles.json");
    const QJsonObject& jsonProfile = jsonProfiles[profile].toObject();

    const QString& scope = jsonProfile["scope"].toString();
    const QString& framework = jsonProfile["framework"].toString();

    const QJsonObject& jsonFrameworks = tools::getJsonObject(APP_CONFIG_PATH "/frameworks.json");
    const QJsonObject& jsonScope = jsonFrameworks[framework].toObject()[scope].toObject();
    const QJsonObject& jsonModels = jsonScope["models"].toObject();

    if (!jsonModels.contains(model))
    {
        throw error::existence::NoSuchModelError();
    }
    
    const QJsonObject& jsonModel = jsonModels[model].toObject();

    const QString& modelPath = DEFAULT_MODELS_PATH "/" + name;

    QMap<QString, QString> replacements;
    replacements.insert("%{NAME}", name);
    replacements.insert("%{MODEL_PATH}", modelPath);

    const QString& script = tools::interpretPath(jsonModel["install_script"].toString(), replacements);

    qDebug() << "\033[32m[INFO]: Install finished with output: " << tools::installProcess(script) << "\033[0m";

    QJsonObject newModel;

    newModel["path"] = modelPath;
    newModel["model"] = model;

    jsonUserModels[name] = newModel;
    jsonProject["models"] = jsonUserModels;
    jsonProjects[project] = jsonProject;

    tools::writeJson(USER_CONFIG_PATH "/projects.json", jsonProjects);
}



void model::trainModel
    (
    const QString& name,
    const QString& project
    )
{
    qDebug() << "\033[90m[DEBUG]: Name is:" << name << "\033[0m";
    qDebug() << "\033[90m[DEBUG]: Project is:" << project << "\033[0m";

    QJsonObject jsonProjects = tools::getJsonObject(USER_CONFIG_PATH "/projects.json");

    if (!jsonProjects.contains(project))
    {
        throw error::existence::NoSuchProjectError();
    }

    QJsonObject jsonProject = jsonProjects[project].toObject();
    QJsonObject jsonModels = jsonProject["models"].toObject();

    if (!jsonModels.contains(name))
    {
        throw error::existence::NoSuchModelError();
    }

    const QJsonObject& jsonModel = jsonModels[name].toObject();

    const QString& profile = jsonProject["profile"].toString();

    QJsonObject jsonProfiles = tools::getJsonObject(USER_CONFIG_PATH "/profiles.json");
    const QJsonObject& jsonProfile = jsonProfiles[profile].toObject();

    const QString& scope = jsonProfile["scope"].toString();
    const QString& framework = jsonProfile["framework"].toString();

    const QJsonObject& jsonFrameworks = tools::getJsonObject(APP_CONFIG_PATH "/frameworks.json");
    const QJsonObject& jsonScope = jsonFrameworks[framework][scope].toObject();

    /*
    Py_Initialize();

    Py_SetPythonHome(jsonProfile["python_path"].toString().toStdWString().c_str());

    QMap<QString, QString> replacements;
    replacements.insert("%{PROFILE_PATH}", profile);

    const QString& script = tools::interpretPath(jsonScope["training_script"].toString(), replacements);

    const char* scriptPath = script.toString().toUtf8().constData();
    FILE* scriptFile = fopen(scriptPath, "r");
    if (scriptFile) {
        PyRun_SimpleFile(scriptFile, scriptPath);
        fclose(scriptFile);
    } else {
        qFatal() << "No script.. is it deleted?";
    }

    Py_Finalize();
    */

    QMap<QString, QString> replacements;
    replacements.insert("%{NAME}", name);
    replacements.insert("%{MODEL_PATH}", jsonModel["path"].toString());

    const QString& script = tools::interpretPath(jsonScope["training_script"].toString());

    qDebug() << "\033[32m[INFO]: Training finished with output: " << tools::installProcess(script) << "\033[0m";
}

void model::deleteModel
    (
    const QString& name,
    const QString& project,
    bool confirmationDialog
    )
{
    QJsonObject jsonProjects = tools::getJsonObject(USER_CONFIG_PATH "/projects.json");

    if (!jsonProjects.contains(project))
    {
        throw error::existence::NoSuchProjectError();
    }

    QJsonObject jsonProject = jsonProjects[project].toObject();

    if (!jsonProjects["models"].toObject().contains(name))
    {
        throw error::existence::NoSuchModelError();
    }

    const QJsonObject& jsonModels = jsonProjects["models"].toObject();

    tools::deleteFromObject(name, jsonModels, confirmationDialog);
}

void model::list
    (
    const QString& framework,
    const QString& scope
    )
{
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

    qInfo().noquote() << tools::list(jsonScope["models"].toObject()).toUtf8();
}
