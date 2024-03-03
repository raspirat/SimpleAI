#include "dataset.h"

#include "../config/config.h"

#include "../utils/tools.h"
#include "../utils/errors.h"

#include <QString>
#include <QSettings>
#include <QFile>
#include <QDir>
#include <QDebug>
#include <QJsonDocument>
#include <QJsonObject>
#include <QProcessEnvironment>

int dataset::createDataset
    (
    const QString& name,
    const QString& labelmapPath,
    const QString& dataPath,
    const QString& labelsPath
    )
{
    using namespace::std;

    QJsonObject jsonDatasets = tools::getJsonObject(USER_CONFIG_PATH "/datasets.json");
    QJsonObject newDataset;

    if (jsonDatasets.contains(name))
    {
        throw error::name::DatasetNameError();
        return -1;
    }

    QProcessEnvironment env = QProcessEnvironment::systemEnvironment();

    QString datasetPath = env.value("DATASETS_PATH");

    if (datasetPath.isEmpty())
    {
        error::environment::DATASETS_PATH_Error error;

        datasetPath = DEFAULT_DATASETS_PATH;

        qInfo() << error.what() << "\033[36m"
                << "Default:" << datasetPath
                << "\033[0m";

        // set the $SA_PROFILE_PATH for debian
        qDebug() << "\033[90m[DEBUG]: Script executed with output:"
                 << tools::installProcess(APP_SCRIPTS_PATH "/set_debian_env.sh DATASETS_PATH " + datasetPath)
                 << "\033[0m";
    }

    datasetPath = datasetPath + "/" + name;

    qInfo() << "\033[32m[INFO]: Your dataset will be stored in:\033[35m" << datasetPath << "\033[0m";

    const QString newDataPath = datasetPath + "/images";
    const QString newLabelsPath = datasetPath + "/labels";

    QSettings settings(APP_CONFIG_PATH "/config.ini", QSettings::IniFormat);

    QStringList label_formats = settings.value("dataset/supported_labeling_formats").toStringList();

    if (tools::copyFilesWithExtention(labelsPath,newLabelsPath,label_formats) == 0)
    {
        throw error::compatibility::LabelExtentionError();
        return -2;
    }

    QStringList img_formats = settings.value("dataset/supported_img_formats").toStringList();

    if (tools::copyFilesWithExtention(dataPath,newDataPath,img_formats) == 0)
    {
        throw error::compatibility::ImageExtentionError();
        return -3;
    }

    const QString& newLabelmapPath = datasetPath + "/annotations/labelmap.pbtxt";

    QFile::copy(labelmapPath, newLabelmapPath);

    newDataset["path"] = datasetPath;
    newDataset["data"] = newDataPath;
    newDataset["labels"] = newLabelsPath;
    newDataset["labelmap"] = newLabelmapPath;

    jsonDatasets[name] = newDataset;

    tools::writeJson(USER_CONFIG_PATH "/datasets.json", jsonDatasets);

    qInfo() << "\033[32m[INFO]: Successfully created dataset!\033[0m";
    return 0;
}

int dataset::deleteDataset
    (
    const QString& name,
    bool confirmationDialog
    )
{
    QJsonObject jsonDatasets = tools::getJsonObject(USER_CONFIG_PATH "/datasets.json");

    if (!jsonDatasets.contains(name))
    {
        throw error::existence::NoSuchDatasetError();
        return -1;
    }

    qInfo() << "\033[32m[INFO]: Deleting dataset...\033[0m";

    tools::deleteFromObject(name, jsonDatasets, confirmationDialog);
    return 0;
}

void dataset::list()
{
    const QJsonObject& jsonDatasets = tools::getJsonObject(USER_CONFIG_PATH "/datasets.json");

    qInfo().noquote() << tools::list(jsonDatasets).toUtf8();
}
