#include "tools.h"

#include "../config/config.h"
#include "../utils/errors.h"

#include <string>
#include <iostream>
#include <exception>

#include <QString>
#include <QSaveFile>
#include <QDir>
#include <QJsonDocument>
#include <QJsonObject>
#include <QDebug>
#include <QProcess>
#include <QMap>
#include <QStandardPaths>
#include <QResource>
#include <QTextStream>

QString tools::getFullPath
    (
    QString path
    )
{
    return QFileInfo(path.replace("~", QDir::homePath())).absoluteFilePath();
}


QJsonObject tools::getJsonObject
    (
    const QString& filename
    )
{
    const QString& fullPath = tools::getFullPath(filename);

    QFile jsonFile(fullPath);

    if (!jsonFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        qCritical() << "\033[36m[ALERT]: Could not find resource file:" << fullPath << "Creating a new one.\033[0m";

        return QJsonObject();
    }

    QByteArray jsonData = jsonFile.readAll();
    jsonFile.close();

    QJsonParseError parsingError;
    QJsonDocument jsonDoc = QJsonDocument::fromJson(jsonData, &parsingError);

    if (parsingError.error != QJsonParseError::NoError) {
        qCritical() << "\033[33m[ERROR] <CRITICAL>: File Syntax wrong or uncompleted! Is the file corrupted?\033[0m";
    }

    QJsonObject jsonObject = jsonDoc.object();

    return jsonObject;
}

void tools::writeJson
    (
    const QString& filename,
    QJsonObject jsonObject
    )
{
    const QString& fullPath = getFullPath(filename);

    QSaveFile jsonFile(fullPath);

    QDir().mkpath(QFileInfo(filename).absolutePath());

    // qDebug() << "\033[90m[DEBUG]: Json file properties:" << jsonFile.fileName() << "\033[0m";

    if (!jsonFile.open(QIODevice::WriteOnly)) {
        qCritical() << "\033[33m[ERROR] <CRITICAL>: Could not create new File!:\033[35m" << fullPath << "\033[0m";
    }

    QJsonDocument jsonDoc(jsonObject);
    jsonFile.write(jsonDoc.toJson());

    if (!jsonFile.commit())
    {
        qFatal() << "\033[31m[ERROR] <FATAL>: Could not save the file! Try again!?\033[0m";
    }

    qInfo() << "\033[32m[INFO]: Successfully saved file!\033[35m" << fullPath << "\033[0m";
}

void tools::updateProgressBar
    (
    int progress,
    int total
    )
{
    const int barWidth = 40;
    int completedWidth = barWidth * progress / total;

    QString progressBar;

    if (progress > 0)
    {
        progressBar += "\033[1A\033[K\033[s";
    }

    progressBar += "\033[34m[PROGRESS]:";
    progressBar += (QString("[%1%2] %3\%").arg(QString(completedWidth, '#'),
                                           QString(barWidth - completedWidth, '.'),
                                           QString::number(100 * progress / total)
                                               )).toLocal8Bit().data();
    progressBar += "\033[0m";

    qInfo().noquote() << progressBar;
}

void tools::createPath
    (
    const QString& path
    )
{
    QString fullPath = tools::getFullPath(path);

    QDir destination(fullPath);

    if (!destination.exists())
    {
        qCritical() << "\033[36m[ALERT]: Could not find the" << fullPath << "directory. Creating a new one.\033[0m";

        if (destination.mkpath(fullPath))
        {
            qInfo() << "\033[32m[INFO]: Successfully created directory!\033[0m";
        } else
        {
            qFatal() << "\033[31m[ERROR] <FATAL>: Failed to create directory!\033[0m";
        }
    }
}

void tools::deleteFromObject
    (
    const QString& name,
    QJsonObject object,
    bool confirmationDialog
    )
{
    if (!object.contains(name))
    {
        throw error::existence::NoSuchDatasetError();
    }

    QJsonObject thisDataset = object[name].toObject();

    const QString& datasetPath = tools::getFullPath(thisDataset["path"].toString());

    qInfo() << "\033[32m[INFO]: Dataset path:" << datasetPath << "\033[0m";

    QTextStream inputStream(stdin);

    if (confirmationDialog)
    {
        qWarning() << "\033[36m[ALERT]: Do you really want to delete" << name << "?\n If so press the y key.\n\033[0m";

        QString userInput = inputStream.readLine();

        if (!(userInput.toLower() == "y"))
        {
            return;
        }
    }

    object.remove(name);
    tools::writeJson(USER_CONFIG_PATH "/datasets.json", object);

    if (!QDir(datasetPath).removeRecursively())
    {
        qFatal() << "\033[31m[ERROR] <FATAL>: Failed to remove directory!" << datasetPath << "\033[0m";
    }

    tools::updateProgressBar(1,1);

    qInfo() << "\033[32m[INFO]: Successfully deleated dataset!\033[0m";
}

int tools::copyFilesWithExtention
    (
    const QString& sourceDir,
    const QString& destDir,
    const QStringList& extensions
    )
{
    QString fullSourceDir = tools::getFullPath(sourceDir);
    QString fullDestDir = tools::getFullPath(destDir);

    tools::createPath(fullDestDir);

    QDir directory(fullSourceDir);

    QStringList filters;

    for (const QString& ext : extensions)
    {
        filters.append("*." + ext);
    }

    qDebug() << "\033[90m[DEBUG]: Filters are:" << filters << "\033[0m";

    QStringList files = directory.entryList(filters, QDir::Files | QDir::NoDotAndDotDot);

    qInfo() << "\033[32m[INFO]:"
            << files.size() << "files will be copied from\033[35m"
            << directory.absolutePath() << "\033[32mto\033[35m"
            << fullDestDir;
    qInfo() << "\033[90m[DEBUG]: Files:" << files.join(", ") << "\033[0m";


    qInfo().noquote() << "\033[32m[INFO]: Only" << extensions.size() << "supported formats:" << extensions.join(", ") << "\033[0m";


    for (int i = 0; i < files.length(); i++)
    {
        QString sourcePath = directory.filePath(files[i]);
        QString destPath = QDir(fullDestDir).filePath(files[i]);

        if (QFile::copy(sourcePath, destPath))
        {
            tools::updateProgressBar(i+1, files.length());
        }
        else
        {
            qCritical() << "\033[33m[ERROR] <CRITICAL>: Failed to copy file:\033[35m" << files[i] << "\033[0m";
        }
    }

    return files.length();
}

QString tools::installProcess
    (
    const QString& script,
    const QStringList& envVars
    )
{
    QStringList scriptParts = script.split(" ");
    QString standaloneScript = scriptParts.first();

    if (!QFile::exists(standaloneScript) || standaloneScript == "")
    {
        qFatal() << "\033[31m[ERROR] <FATAL>: Could not find script! Path:" << standaloneScript << "\033[0m";
    }

    qDebug() << "\033[90m[DEBUG]: Using standalone script:" << standaloneScript << "\033[0m";
    qDebug() << "\033[90m[DEBUG]: Using script:" << script << "\033[0m";

    QString terminal = "/bin/bash";

    QStringList arguments;
    arguments << "-i" << "-c" << "source " + script;

    qDebug() << "\033[90m[DEBUG]: Using arguments:" << arguments << "\033[0m";

    QFile::setPermissions(standaloneScript, QFile::ReadOwner | QFile::WriteOwner | QFile::ExeOwner);

    QProcess installationProcess;

    installationProcess.setEnvironment(envVars);

    const QString& workDir = QDir::homePath()+"/."+QCoreApplication::applicationName()+"/tmp";
    tools::createPath(workDir);

    installationProcess.setWorkingDirectory(workDir);

    qDebug() << "\033[90m[DEBUG]: Work dir:" << installationProcess.workingDirectory() << "\033[0m";

    installationProcess.start(terminal, arguments);

    qInfo() << "\033[32m[INFO]: Live output:\033[90m";

    // QString pattern = "\\n *\\d+ +\\d+M +([0-9]+)";
    // QString condaProgressPattern = " *([0-9]+) *\\d+M";

    int errorCount = 0;

    QObject::connect(
        &installationProcess,
        &QProcess::readyReadStandardOutput,
        [&]() {
            QByteArray outputData = installationProcess.readAllStandardOutput();
            qDebug().noquote() << "\033[35m[SCRIPT_INFO]:" << outputData.toStdString() << "\033[90m";
        });

    QObject::connect(
        &installationProcess,
        &QProcess::readyReadStandardError,
        [&]() {
            QString errorData = QString::fromUtf8(installationProcess.readAllStandardError());
            qCritical().noquote() << errorData;
            if (errorData.contains("error",Qt::CaseInsensitive))
            {
                errorCount += 1;
            }
        });

    qInfo() << "\033[0m";

    if (installationProcess.waitForFinished(-1) && errorCount == 0)
    {
        QByteArray output = installationProcess.readAllStandardOutput();
        return QString(output);
    }
    qFatal() << "\033[31m[ERROR] <FATAL>: Script errored! Count(" << errorCount << "):" << installationProcess.readAllStandardError() << "\033[0m";
    return QString();
}

QString tools::interpretPath
    (
    const QString& path,
    QMap<QString, QString> replacements
    )
{
    qDebug() << "\033[90m[DEBUG]: %{APP_SCRIPTS_PATH}: " << APP_SCRIPTS_PATH;

    replacements.insert("%{APP_SCRIPTS_PATH}", tools::getFullPath(APP_SCRIPTS_PATH));
    replacements.insert("%{APP_CONFIG_PATH}", tools::getFullPath(APP_CONFIG_PATH));
    replacements.insert("%{USER_CONFIG_PATH}", tools::getFullPath(USER_CONFIG_PATH));
    replacements.insert("%{USER_SCRIPTS_PATH}", tools::getFullPath(USER_SCRIPTS_PATH));
    replacements.insert("%{DEFAULT_DATASETS_PATH}", tools::getFullPath(DEFAULT_DATASETS_PATH));
    replacements.insert("%{DEFAULT_PROFILES_PATH}", tools::getFullPath(DEFAULT_PROFILES_PATH));
    replacements.insert("%{DEFAULT_PROJECTS_PATH}", tools::getFullPath(DEFAULT_PROJECTS_PATH));
    replacements.insert("%{DEFAULT_MODELS_PATH}", tools::getFullPath(DEFAULT_MODELS_PATH));
    replacements.insert("%{APP_DATA_PATH}", tools::getFullPath(APP_DATA_PATH));


    QString result = path;

    for (auto it = replacements.constBegin(); it != replacements.constEnd(); ++it)
    {
        result.replace(it.key(), it.value());
    }

    return getFullPath(result);
}

const QString tools::list
    (
    const QJsonObject& object
    )
{
    QString outStr;

    if (!object.isEmpty())
    {
        for (const QString& objectName : object.keys())
        {
            QJsonObject childObject = object.value(objectName).toObject();

            outStr += "[" + objectName + "]\n"
                + "  |" + "\n";

            for (const QString& childObjectName : childObject.keys())
            {
                if (!childObject.value(childObjectName).isObject())
                {
                    outStr += "  |--(" + childObjectName + ")-->" + childObject.value(childObjectName).toString() + "\n";
                }
                else
                {
                    outStr += "  |--[" + childObjectName + "]-+\n";
                }
            }
            outStr += QString() + "  |" + "\n";
        }
        outStr += QString() +  "  °" + "\n";
    }
    else
    {
        qInfo() << "\033[32m[INFO]: Nothing to list.\033[0m";
    }
    return outStr;
}


