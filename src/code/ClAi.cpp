#include "include/code/ClAi.hpp"

ClAi::ClAi() {}

const QString ClAi::getDatasetsConfigPath()
{
    return tools::getFullPath(USER_CONFIG_PATH "/datasets.json");
}

const QString ClAi::getProfilesConfigPath()
{
    return tools::getFullPath(USER_CONFIG_PATH "/profiles.json");
}

const QString ClAi::getProjectsConfigPath()
{
    return tools::getFullPath(USER_CONFIG_PATH "/projects.json");
}

QJsonObject ClAi::getDatasetsJson()
{
    return tools::getJsonObject(ClAi::getDatasetsConfigPath());
}

QJsonObject ClAi::getProfilesJson()
{
    return tools::getJsonObject(ClAi::getProfilesConfigPath());
}

QJsonObject ClAi::getProjectsJson()
{
    return tools::getJsonObject(ClAi::getProjectsConfigPath());
}

QJsonObject ClAi::getModelsJson()
{
    return ClAi::getProjectsJson()["models"].toObject();
}

bool ClAi::saveJson(const QJsonObject & object, const QString & filePath)
{
    try
    {
        tools::writeJson(filePath, object); return true;
    }
    catch(std::exception err)
    {
        return false;
    }
}

int ClAi::createProject(const QString &name, const QString &profile, const QString &dataset) {
    project::createProject(name, profile, dataset);
    return 0;
}

int ClAi::deleteProject(const QString &name) {
    project::deleteProject(name, false);
    return 0;
}

int ClAi::createProfile(const QString &name, const QString &framework, const QString &scope) {
    profile::createProfile(name, framework, scope);
    return 0;
}

int ClAi::deleteProfile(const QString &name) {
    profile::deleteProfile(name, false);
    return 0;
}

int ClAi::createModel(const QString &name,const  QString &project, const QString &model) {
    model::createModel(name, project, model);
    return 0;
}

int ClAi::deleteModel(const QString &name, const QString &project) {
    model::deleteModel(name, project, false);
    return 0;
}

int ClAi::trainModel(const QString &name, const QString &project, const QString &args) {
    model::trainModel(name, project, args);
    return 0;
}

int ClAi::deleteDataset(const QString &name) {
    dataset::deleteDataset(name, false);
    return 0;
}

int ClAi::createDataset(const QString &name, const QString &dataPath, const QString &labelsPath) {
    dataset::createDataset(name, dataPath, labelsPath);
    return 0;
}
