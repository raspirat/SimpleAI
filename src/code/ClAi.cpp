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

int ClAi::createProject(QString &name, QString &profile, QString &dataset) {
    return project::createProject(name, profile, dataset);
}

int ClAi::deleteProject(QString &name) {
    return project::deleteProject(name, false);
}

int ClAi::createProfile(QString &name, QString &framework, QString &scope) {
    return profile::createProfile(name, framework, scope);
}

int ClAi::deleteProfile(QString &name) {
    return profile::deleteProfile(name, false);
}

int ClAi::createModel(QString &name, QString &project, QString &model) {
    return model::createModel(name, project, model);
}

int ClAi::deleteModel(QString &name, QString &project) {
    return model::deleteModel(name, project, false);
}

int ClAi::trainModel(QString &name, QString &project) {
    return model::trainModel(name, project);
}

int ClAi::deleteDataset(QString &name) {
    return dataset::deleteDataset(name, false);
}

int ClAi::createDataset(QString &name, QString &labelmapPath, QString &dataPath, QString &labelsPath) {
    return dataset::createDataset(name, labelmapPath, dataPath, labelsPath);
}
