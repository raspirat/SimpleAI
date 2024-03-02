#include "include/code/ClAi.hpp"

ClAi::ClAi() {

}

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
